const images = document.getElementsByTagName("li");

for (const image of images) {
    image.addEventListener("click", () => {
        const slug = image.firstChild.innerHTML;
        const link = "/users/" + slug;
        document.location.href = link;
    });
};
