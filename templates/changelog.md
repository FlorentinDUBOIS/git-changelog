# Changelog

{%- for repository in repositories %}

## {{ repository.name }}
{% for commit in repository.commits %}
- {{ commit.hash }} >>> {{ commit.date }} | {{ commit.message }} [{{ commit.author }}]
{%- endfor %}
{%- endfor %}
