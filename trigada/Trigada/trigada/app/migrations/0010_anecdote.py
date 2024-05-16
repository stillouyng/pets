# Generated by Django 5.0.6 on 2024-05-15 13:20

import django.db.models.deletion
from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('app', '0009_musician_slug_tab_slug'),
    ]

    operations = [
        migrations.CreateModel(
            name='Anecdote',
            fields=[
                ('id', models.BigAutoField(auto_created=True, primary_key=True, serialize=False, verbose_name='ID')),
                ('title', models.CharField(max_length=250, verbose_name='Title')),
                ('text', models.TextField(verbose_name='Anecdote')),
                ('added', models.DateTimeField(auto_now_add=True, verbose_name='Date added')),
                ('writer', models.ForeignKey(on_delete=django.db.models.deletion.PROTECT, to='app.person', verbose_name='Writer')),
            ],
            options={
                'verbose_name': 'Anecdote',
                'verbose_name_plural': 'Anecdotes',
            },
        ),
    ]
