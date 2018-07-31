# Changelog
{% for repository in repositories %}
## {{ repository.name }}
{% for kind in repository.kinds %}
### {{ kind.name }}
{% for commit in kind.commits %}
- {{ commit.hash }} >>> {{ commit.date }} | {{ commit.message }} [{{ commit.author }}]
{%- endfor %}
{% endfor %}
{%- endfor -%}