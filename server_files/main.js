function open_dropdown_menu() {
    document.getElementById("dropdown-content").style.display = "block";
}

window.onclick = function(event) {
    if (!event.target.matches(".dropdown-button")) {
        document.getElementById("dropdown-content").style.display = "none";
    }
}
