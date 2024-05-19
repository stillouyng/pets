from django.contrib.auth.decorators import login_required
from django.core.exceptions import ObjectDoesNotExist
from django.http import JsonResponse
from django.shortcuts import render
from . import models
from . import forms


def main_view(request):
    makers = models.Person.objects.all()
    return render(request, template_name="main.html", context={"makers": makers})


def persons_view(request, slug: str):
    makers = models.Person.objects.get(slug=slug)
    return render(request, template_name="makers.html", context={"makers": makers})


@login_required
def user_profile(request):
    profile = models.Person.objects.get(user=request.user)
    form = forms.UploadMusicForm()
    return render(request, template_name="registration/profile.html", context={'profile': profile, 'music_form': form})


@login_required
def anecdote_view(request):
    anecdotes = models.Anecdote.objects.all()
    form = forms.AnecdoteForm()
    context = {"anecdotes": anecdotes, "form": form}
    return render(request, template_name="anecdotes.html", context=context)


@login_required
def video_view(request):
    videos = models.Video.objects.all()
    return render(request, template_name="videos.html", context={"videos": videos})


def musician_view(request, slug: str):
    context: dict = {}
    try:
        musician = models.Musician.objects.get(slug=slug)
        if musician:
            context['musician'] = musician
            tabs = models.Tab.objects.filter(musician=musician)
            if tabs:
                context['songs'] = tabs
    except ObjectDoesNotExist:
        pass
    return render(request, template_name="musician.html", context=context)


def song_view(request):
    global tabs
    if "query" in request.POST:
        form = forms.SearchTabsForm(request.POST)
        if form.is_valid():
            data = form.cleaned_data
            tabs = models.Tab.objects.filter(song_name__contains=data['query'])
            if not tabs:
                musician = models.Musician.objects.filter(username__contains=data['query'])
                if musician:
                    tabs = models.Tab.objects.filter(musician=musician[0])
    else:
        form = forms.SearchTabsForm()
        tabs = models.Tab.objects.all()
    return render(request, template_name="songs.html", context={"tabs": tabs, "form": form})


def tab_view(request, slug: str):
    try:
        tab = models.Tab.objects.get(slug=slug)
        context = {'tab': tab}
    except ObjectDoesNotExist:
        context = {}
    return render(request, template_name="tabs.html", context=context)


def blog_view(request):
    blogs = models.Blog.objects.all()
    form = forms.BlogForm()
    return render(request, template_name="blog.html", context={"blogs": blogs, "form": form})


def anecdote_form_view(request):
    context: dict = {}
    if request.method == "POST":
        form = forms.AnecdoteForm(request.POST)
        if form.is_valid():
            data = form.cleaned_data
            anecdote = models.Anecdote(
                title=data.get('title'),
                text=data.get('text'),
                writer=models.Person.objects.get(user=request.user)
            )
            anecdote.save()
            context['text'] = "Anecdote was added successfully!"
        else:
            context['text'] = "There is some error :("
        context['previous_page'] = request.POST.get("previous_page")
    return render(request, template_name="success_form.html", context=context)


def forms_blog_view(request):
    context: dict = {}
    if request.method == "POST":
        form = forms.BlogForm(request.POST)
        if form.is_valid():
            data = form.cleaned_data
            blog = models.Blog(
                title=data.get('title'),
                text=data.get('text'),
                writer=models.Person.objects.get(user=request.user)
            )
            blog.save()
            context['text'] = "Blog was added successfully!"
        else:
            context['text'] = 'There is some error :('
        context['previous_page'] = request.POST.get("previous_page")
    return render(request, template_name="success_form.html", context=context)


def forms_song_view(request):
    context: dict = {}
    if request.method == "POST":
        form = forms.UploadMusicForm(request.POST, request.FILES)
        if form.is_valid():
            user = models.Person.objects.get(slug=request.POST.get("slug"))
            user.song = request.FILES['song']
            user.save()
            context['text'] = "Song was uploaded successfully!"
        else:
            context['text'] = "There is some error :("
        context['previous_page'] = request.POST.get("previous_page")
    return render(request, template_name="success_form.html", context=context)
