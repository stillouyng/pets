from django.urls import path
from . import views

api_urlpatterns = [
    path("api/v1/song_form_success/", views.forms_song_view, name="song_form"),
    path("api/v1/blog_form_success/", views.forms_blog_view, name="blog_form"),
    path("api/v1/anecdote_form_success/", views.anecdote_form_view, name="anecdote_form"),
]

urlpatterns = [
    path("", views.main_view, name='main'),
    path("users/<slug:slug>", views.persons_view, name='users'),
    path("accounts/profile/", views.user_profile, name='profiles'),
    path("videos/", views.presentation_video_view, name='presentation_video'),
    path("all_videos/", views.video_view, name="videos"),
    path("songs/", views.song_view, name='songs'),
    path("tabs/<slug:slug>", views.tab_view, name='tabs'),
    path("musician/<slug:slug>", views.musician_view, name="musicians"),
    path("anecdotes/", views.anecdote_view, name='anecdotes'),
    path("blog/", views.blog_view, name='blogs'),
] + api_urlpatterns
