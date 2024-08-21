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
    const items = event.target.parentNode.parentNode.parentNode.children
    for(const item of items) {
        if(item.localName == "ul") {
            for(const child of item.children) {
                child.classList.toggle("hidden")
            }
        }
    }
}

const handleThemeSwitch = (event) => {
    const newTheme = document.documentElement.classList.contains(THEME_CLASS_DARK) ? THEME_CLASS_LIGHT : THEME_CLASS_DARK
    localStorage.setItem(LOCAL_STORAGE_KEY_THEME, newTheme)
    setTheme()
}

const main = () => {
    document.getElementById("navigation-button").addEventListener("click", handleClick)

    const toggles = document.getElementsByClassName("nav-toggle")
    for(const item of toggles) {
        item.addEventListener("click", handleNavClick);
    }

    document.getElementById("theme-switch").addEventListener("click", handleThemeSwitch)
}

window.onload = main;