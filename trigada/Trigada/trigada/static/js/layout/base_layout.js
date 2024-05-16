const menuButton = document.getElementById("menu-button");
const menuWindow = document.getElementById("menu-window");


menuButton.onclick = function(){
	this.classList.toggle('menu-rotate');
	if (menuWindow.classList.contains('menu-hider')) {
	    menuWindow.classList.remove('menu-hider');
	}
	menuWindow.classList.toggle('menu-show');
	if (!(menuWindow.classList.contains('menu-show'))) {
	    menuWindow.classList.add('menu-hider');
	}
}
