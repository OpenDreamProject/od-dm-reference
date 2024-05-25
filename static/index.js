const handleClick = () => {
    const element = document.getElementById("left-bar");
    if(element.style.display == "block") {
        element.classList.remove("animate-in");
        element.classList.add("animate-out");
        setTimeout(() => {element.style.display = "none"}, 250)
    } else {
        element.style.display = "block";
        element.classList.remove("animate-out");
        element.classList.add("animate-in");
    }
    
}

const main = () => {
    document.getElementById("navigation-button").addEventListener("click", handleClick)
}

window.onload = main;