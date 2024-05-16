from django.urls import path
from . import views

api_urlpatterns = [
]

urlpatterns = [
    path("", views.main_view, name='main'),
    path("users/<slug:slug>", views.persons_view, name='users'),
    path("accounts/profile/", views.user_profile, name='profiles'),
    path("videos/", views.video_view, name='videos'),
    path("songs/", views.song_view, name='songs'),
    path("tabs/<slug:slug>", views.tab_view, name='tabs'),
    path("musician/<slug:slug>", views.musician_view, name="musicians"),
    path("anecdotes/", views.anecdote_view, name='anecdotes'),
    path("blog/", views.blog_view, name='blogs'),
] + api_urlpatterns
