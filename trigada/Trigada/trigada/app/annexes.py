

def handle_uploaded_songs(file) -> str:
    filepath = f"/media/makers/songs/{file.name}"
    with open(filepath, 'wb+') as destination:
        for chunk in file.chunks():
            destination.write(chunk)
    return filepath
