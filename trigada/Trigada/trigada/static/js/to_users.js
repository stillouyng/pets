const images = document.getElementsByTagName("img");

for (const image of images) {
    if (image.classList.contains("maker-image")) {
        image.addEventListener("click", () => {
            slug = image.id;
            const link = "/users/" + slug;
            document.location.href = link;
        });
    };
};
