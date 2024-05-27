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

const handleNavClick = (event) => {
    event.target.classList.toggle("fa-rotate-90")
    const items = event.target.parentNode.children
    for(const item of items) {
        if(item.localName == "ul") {
            for(const child of item.children) {
                child.classList.toggle("hidden")
            }
        }
    }
}

const main = () => {
    document.getElementById("navigation-button").addEventListener("click", handleClick)

    const toggles = document.getElementsByClassName("nav-toggle")
    for(const item of toggles) {
        item.addEventListener("click", handleNavClick);
    }

}

window.onload = main;