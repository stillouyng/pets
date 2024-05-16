const writeAnAnecdote = document.getElementById("write-an-anecdote");
const anecdoteForm = document.getElementById("anecdote-form");

writeAnAnecdote.addEventListener("click", () => {
    anecdoteForm.classList.remove('hidden');
});

const toggleForm = () => anecdoteForm.classList.toggle("hidden");

window.onclick = (event) => {
    if (!event.target.matches('#write-an-anecdote')) {
        if(!(anecdoteForm.classList.contains('hidden'))){
            anecdoteForm.classList.add('hidden')
        }
    }
}

anecdoteForm.addEventListener('click', (event) => event.stopPropagation());
