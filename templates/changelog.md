# Changelog
{% for repository in repositories %}
## {{ repository.name }}
{% for kind in repository.kinds %}
### {{ kind.name }}
{% for commit in kind.commits %}
{% match commit.link -%}
  {%- when Some with (link) -%}
- [ [`{{ commit.hash }}`]({{ link }}) ] {{ commit.message }} [`{{ commit.author }}`] (`{{ commit.date }}`)
  {%- when None -%}
- [ `{{ commit.hash }}` ] {{ commit.message }} [`{{ commit.author }}`] (`{{ commit.date }}`)
{%- endmatch -%}
{% endfor %}
{% endfor %}
{%- endfor -%}
