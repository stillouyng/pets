// Variables
const additionalInfoBlock = document.getElementById("additional-information");
const toggleAdditionalButton = document.getElementById("additional-block");
const toAdminPage = document.getElementById("to-admin-page");

// Event listener
toggleAdditionalButton.addEventListener("click", ()=>{
    toggleAdditionalButton.classList.toggle("svg-arrow-rotated");
    additionalInfoBlock.classList.toggle("add-info-show");
});

toAdminPage.addEventListener("click", () => {
    window.location.href = "/admin/";
});