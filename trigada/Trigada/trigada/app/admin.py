from django.contrib import admin
from . import models


admin.site.register(models.Person)
admin.site.register(models.Video)
admin.site.register(models.Tab)
admin.site.register(models.Musician)
admin.site.register(models.Anecdote)
admin.site.register(models.Blog)
