<%namespace name="util" file="../../lib/util.mako"/>\
<%!
    from mako.filters import xml_escape
    from util import (hash_comment, new_context, method_default_scope, indent_all_but_first_by, is_repeated_property)
    from cli import (subcommand_md_filename, new_method_context, SPLIT_START, SPLIT_END, pretty, SCOPE_FLAG,
                     mangle_subcommand, is_request_value_property, FIELD_SEP, PARAM_FLAG, UPLOAD_FLAG, docopt_mode,
                     FILE_ARG, MIME_ARG, OUT_ARG, OUTPUT_FLAG, to_cli_schema, cli_schema_to_yaml, SchemaEntry,
                     STRUCT_FLAG, field_to_value, CTYPE_ARRAY, CTYPE_MAP)

    from copy import deepcopy

    escape_html = lambda n: n.replace('>', r'\>')

    NO_DESC = 'No description provided.'
%>\
<%
    c = new_context(schemas, resources, context.get('methods'))
%>\
% for resource in sorted(c.rta_map.keys()):
% for method in sorted(c.rta_map[resource]):
<%
    mc = new_method_context(resource, method, c)
%>\
${SPLIT_START} ${subcommand_md_filename(resource, method)}
% if 'description' in mc.m:
${mc.m.description | xml_escape}
% endif # show method description
% if mc.m.get('scopes'):
# Scopes

You will need authorization for \
% if len(mc.m.scopes) > 1:
at least one of the following scopes to make a valid call:

% for s in mc.m.scopes:
* *${s}*
% endfor
% else:
the *${mc.m.scopes[0]}* scope to make a valid call.
% endif # len(scopes) > 1

If unset, the scope for this method defaults to *${method_default_scope(mc.m)}*.
You can set the scope for this method like this: `${util.program_name()} --${SCOPE_FLAG} <scope> ${mangle_subcommand(resource)} ${mangle_subcommand(method)} ...`
% endif # have method scopes
<%
    rprops = [p for p in mc.required_props if not is_request_value_property(mc, p)]
    oprops = [p for p in mc.optional_props if not p.get('skip_example', False)]

    smd = mc.m.get('supportsMediaDownload', False)
%>\
% if rprops:
# Required Scalar ${len(rprops) > 1 and 'Arguments' or 'Argument'}
% for p in rprops:
* **<${mangle_subcommand(p.name)}\>**
    - ${p.get('description') or NO_DESC | xml_escape, indent_all_but_first_by(2)}
% endfor  # each required property (which is not the request value)
% endif # have required properties
% if mc.request_value:
<%
    request_cli_schema = to_cli_schema(c, mc.request_value)
%>\
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
${cli_schema_to_yaml(request_cli_schema)}
```

can be set completely with the following arguments. Note how the cursor position is adjusted to the respective fields:

${self._list_schem_args(request_cli_schema)}

* The cursor position is always set relative to the current one, unless the field name starts with the '${FIELD_SEP}' character. Fields can be nested such as in `-r f${FIELD_SEP}s${FIELD_SEP}o` .
* **Lists** are always appended to, in the example, the list at `struct${FIELD_SEP}sub_struct${FIELD_SEP}strings` will have the value `["first", "second"]`.
* **Mappings** are set using the `key=value` form.
* You can also set nested fields without setting the cursor explicitly. For example, to set the mapping from the root, you would specify `-r struct${FIELD_SEP}sub_struct${FIELD_SEP}mapping=foo=bar`. In case the cursor is not at the root, you may explicitly drill down from the root using a leading '${FIELD_SEP}' character.

% endif # have request value
% if mc.media_params:
<%
    protocols = [mp.protocol for mp in mc.media_params]
%>\
# Required Upload Flags

This method supports the upload of data, using the following protocol${len(mc.media_params) > 1 and 's' or ''}:

* **-${UPLOAD_FLAG} ${docopt_mode(protocols)} ${escape_html(FILE_ARG)} ${escape_html(MIME_ARG)}**
% for mp in mc.media_params:
    - **${mp.protocol}** - ${mp.get('description', NO_DESC).split('\n')[0] | xml_escape}
% endfor # each media param
    - **${escape_html(FILE_ARG)}**
        + Path to file to upload. It must be seekable.
    - **${escape_html(MIME_ARG)}**
        + the mime type, like 'application/octet-stream', which is the default
% endif # have upload capabilities
% if mc.response_schema or smd:
# Optional Output Flags

% if mc.response_schema:
The method's return value a JSON encoded structure, which will be written to standard output by default.
% endif
% if smd:

% if mc.response_schema:
As this method supports **media download**, you may specify the `-${PARAM_FLAG} alt=media` flag to set the output to be an octet stream of the underlying media. In that case, you will not receive JSON output anymore.
% else:
The method's return value is a byte stream of the downloadable resource.
% endif # handle response schema
% endif # support media download

* **-${OUTPUT_FLAG} ${escape_html(OUT_ARG)}**
    - *${escape_html(OUT_ARG)}* specifies the *destination* to which to write the server's result to.
% if smd and mc.response_schema:
      It will either be a JSON-encoded structure, or the media file you are downloading.
% elif smd:
      It will be a byte stream of the downloadable resource.
% else:
      It will be a JSON-encoded structure.
% endif
      The *destination* may be `-` to indicate standard output, or a filepath that is to contain the received bytes.
      If unset, it defaults to standard output.
% endif # have output
% if oprops:
# Optional Method Properties

You may set the following properties to further configure the call.

% for p in sorted(oprops):
${self._md_property(p)}
% endfor 
% endif # optional method properties
% if parameters is not UNDEFINED:
# Optional General Properties

The following properties can configure any call, and are not specific to this method.

% for pn in sorted(parameters.keys()):
<% 
    p = deepcopy(parameters[pn])
    p.name = pn
%>\
${self._md_property(p)}
% endfor 
% endif # general parameters
${SPLIT_END}
% endfor # each method
% endfor # each resource

<%def name="_md_property(p)">\
* **-${PARAM_FLAG} ${mangle_subcommand(p.name)}=${p.type}**
    - ${p.get('description') or NO_DESC | xml_escape ,indent_all_but_first_by(2)}
</%def>

<%def name="_list_schem_args(schema, abs_cursor='')">\
<%
    def cursor_fmt(cursor):
        return '-%s %s ' % (STRUCT_FLAG, cursor)

    def cursor_arg():
        if abs_cursor:
            return cursor_fmt(abs_cursor)
        return ''
    abs_cursor_arg = cursor_arg()
%>\
% for fn in sorted(schema.fields.keys()):
<% 
    f = schema.fields[fn] 
    cursor_prefix = ''
    if abs_cursor == '':
        cursor_prefix = FIELD_SEP
%>\
% if isinstance(f, SchemaEntry):
* **${abs_cursor_arg}-${STRUCT_FLAG} ${mangle_subcommand(fn)}=${field_to_value(f)}**
    - ${f.property.get('description', NO_DESC) | xml_escape, indent_all_but_first_by(2)}
% if f.container_type == CTYPE_ARRAY:
    - Each invocation of this argument appends the given value to the array.
% elif f.container_type == CTYPE_MAP:
    - the value will be associated with the given `key`
% endif # handle container type
<% abs_cursor_arg = '' %>
% else:
${self._list_schem_args(f, '%s%s' % (cursor_prefix, mangle_subcommand(fn)))}
<% abs_cursor_arg = cursor_fmt(FIELD_SEP + FIELD_SEP) %>\
% endif
% endfor
</%def>