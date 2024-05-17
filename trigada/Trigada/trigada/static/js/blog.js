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