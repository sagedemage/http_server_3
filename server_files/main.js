function open_dropdown_menu() {
    document.getElementById("dropdown-content").style.display = "block";
}

window.onclick = function(event) {
    if (!event.target.matches(".dropdown-button")) {
        document.getElementById("dropdown-content").style.display = "none";
    }
}

async function get_data() {
    const url = "http://127.0.0.1:3000/data";
    try {
        const response = await fetch(url, {
            method: "GET",
            headers: {
                "Content-Type": "application/json"
            }
        });

        const result = await response.json();
        console.log(result);
    } catch(error) {
        console.error(error.message);
    }
}

get_data()
