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


class UploadMusicForm(ModelForm):
    error_css_class = "error-uploading-song transparent"
    required_css_class = "form-text transparent"
    song = forms.FileField(
        label="Upload New Maker Song",
        widget=forms.FileInput(
            attrs={
                "class": "song-upload-input oxanium transparent"
            }
        )
    )

    class Meta:
        model = models.Person
        fields = ("song", )


class AnecdoteForm(ModelForm):

    class Meta:
        model = models.Anecdote
        fields = ("title", "text")


class BlogForm(ModelForm):

    class Meta:
        model = models.Blog
        fields = ("title", "text")
