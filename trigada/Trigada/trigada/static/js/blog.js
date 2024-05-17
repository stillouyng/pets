const blogs = document.getElementsByClassName("blog-block");

Array.prototype.forEach.call(blogs, function(blog){
    blog.addEventListener("click", () => {
        blog.style.height = "auto";
    });
});
