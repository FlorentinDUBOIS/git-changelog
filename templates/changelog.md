# Changelog
{% for repository in repositories %}
## {{ repository.name }}
{% for tag in repository.tags %}
### {{ tag.name }}
{% for (kind, commits) in tag.commits %}
#### {{ kind }}
{% for commit in commits %}
{% match commit.link -%}
  {%- when Some with (link) -%}
- [ [`{{ commit.hash }}`]({{ link }}) ] {{ commit.message }} [`{{ commit.author }}`] (`{{ commit.date }}`)
  {%- when None -%}
- [ `{{ commit.hash }}` ] {{ commit.message }} [`{{ commit.author }}`] (`{{ commit.date }}`)
{%- endmatch -%}
{% endfor %}
{% endfor %}
{% endfor %}
{%- endfor -%}
