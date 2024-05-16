from django import forms
from django.forms import ModelForm

from . import models


class SearchTabsForm(forms.Form):
    error_css_class = "error-search-input"
    required_css_class = "form-search"
    query = forms.CharField(
        label="Search",
        required=True,
        widget=forms.TextInput(
            attrs={
                "class": "search-input oxanium transparent",
                "placeholder": "Search"
            }
        )
    )


class AnecdoteForm(ModelForm):

    class Meta:
        model = models.Anecdote
        fields = ("title", "text")
