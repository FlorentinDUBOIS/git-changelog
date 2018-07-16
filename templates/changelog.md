# Changelog
{% for commit in commits %}
- {{ commit.hash }} >>> {{ commit.date }} | {{ commit.message }} [{{ commit.author }}]
{%- endfor %}
