const writeABlog = document.getElementById("write-a-blog");
const blogForm = document.getElementById("blog-form");
const blogs = document.getElementsByClassName("blog-block");

Array.prototype.forEach.call(blogs, function(blog){
    blog.addEventListener("click", () => {
        blog.style.height = "auto";
    });
});

document.addEventListener("click", (event) => {
    if (event.target.classList.contains("blog")) {
        console.log('blog clicked!');
    }
});

// Open/Close blog form

writeABlog.addEventListener("click", () => {
    blogForm.classList.remove('hidden');
});

const toggleForm = () => blogForm.classList.toggle("hidden");

window.onclick = (event) => {
    if (!event.target.matches('#write-a-blog')) {
        if(!(blogForm.classList.contains('hidden'))){
            blogForm.classList.add('hidden')
        }
    }
}

blogForm.addEventListener('click', (event) => event.stopPropagation());
