from django.contrib.auth.models import User
from django.core.validators import FileExtensionValidator
from django.db import models
from embed_video.fields import EmbedVideoField
from ckeditor_uploader import fields
import os


class Person(models.Model):
    user = models.OneToOneField(User, on_delete=models.CASCADE)
    is_maker = models.BooleanField(verbose_name="Is Maker")
    image = models.ImageField(verbose_name="Avatar", default="makers/image/default.png", upload_to="makers/image/")
    song = models.FileField(
        verbose_name="Song",
        default="makers/songs/Derek_Pope_-_Encore.mp3",
        upload_to="makers/songs/",
        validators=[FileExtensionValidator(allowed_extensions=['mp3'])])
    slug = models.CharField(verbose_name="Slug", max_length=100)

    def __str__(self):
        return f"{self.user.username}"

    def get_filename(self):
        return os.path.basename(self.song.name).replace("_", " ")

    class Meta:
        verbose_name = "Person"
        verbose_name_plural = "Persons"


class Video(models.Model):
    title = models.CharField(verbose_name="Title", max_length=100)
    added = models.DateTimeField(verbose_name="Date added", auto_now_add=True)
    users = models.ManyToManyField(verbose_name="Users", to="Person")
    url = EmbedVideoField(verbose_name="Url")

    def __str__(self):
        return self.title

    class Meta:
        verbose_name = "Video"
        verbose_name_plural = "Videos"


class Musician(models.Model):
    first_name = models.CharField(verbose_name="First name", max_length=100, default="", blank=True)
    username = models.CharField(verbose_name="Username", max_length=100, default="", blank=True)
    last_name = models.CharField(verbose_name="Last name", max_length=100, default="", blank=True)
    image = models.FileField(verbose_name="Image", default="musicians/avatars/default.png", upload_to="musicians/avatars/")
    slug = models.CharField(verbose_name="Slug", max_length=100, default="", blank=True)

    def __str__(self):
        return self.username

    class Meta:
        verbose_name = "Musician"
        verbose_name_plural = "Musicians"


class Tab(models.Model):
    musician = models.ForeignKey(verbose_name="Musician", to="Musician", on_delete=models.PROTECT)
    song_name = models.CharField(verbose_name="Song", max_length=100, default="–")
    year = models.CharField(verbose_name="Year", max_length=4, default="", blank=True)
    album = models.CharField(verbose_name="Album", max_length=100, default="", blank=True)
    comments = models.TextField(verbose_name="Comments", default="", blank=True)
    users = models.ManyToManyField(verbose_name="Users", to="Person")
    added = models.DateTimeField(verbose_name="Date added", auto_now_add=True)
    file = models.FileField(verbose_name="File", upload_to="tabs/")
    slug = models.CharField(verbose_name="Slug", max_length=100, default="", blank=True)

    def __str__(self):
        return f"{self.musician} - {self.song_name}"

    def get_filename(self):
        return os.path.basename(self.file.name)

    class Meta:
        verbose_name = "Tab"
        verbose_name_plural = "Tabs"


class Anecdote(models.Model):

    title = models.CharField(verbose_name="Title", max_length=250)
    text = fields.RichTextUploadingField(verbose_name="Anecdote")
    added = models.DateTimeField(verbose_name="Date added", auto_now_add=True)
    writer = models.ForeignKey(verbose_name="Writer", to='Person', on_delete=models.PROTECT)

    def __str__(self):
        return self.title

    class Meta:
        verbose_name = "Anecdote"
        verbose_name_plural = "Anecdotes"


class Blog(models.Model):

    title = models.CharField(verbose_name="Title", max_length=250)
    text = fields.RichTextUploadingField(verbose_name="Text")
    added = models.DateTimeField(verbose_name="Date added", auto_now_add=True)
    user = models.ForeignKey(verbose_name="Contributor", to='Person', on_delete=models.PROTECT)

    def __str__(self):
        return self.title

    class Meta:
        verbose_name = "Blog"
        verbose_name_plural = "Blogs"
