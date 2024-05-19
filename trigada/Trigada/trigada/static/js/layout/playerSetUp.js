const player = new Plyr('#player');
player.volume = 0.05;

window.addEventListener("load", () => {
    // turn off preloader
    document.getElementById("preloader").style.display = "none";

    // Get song name. This is the biggest piece of dog shit...
    const song_name = player.source.split("/")[player.source.split("/").length - 1].split("-")[1].replace("_", "").split(".")[0];
    // Set currentTime and play
    try {
        player.currentTime = parseInt(JSON.parse(localStorage.getItem(song_name))['currentTime']);
    } catch (err) {
        player.currentTime = 0;
    }
    player.play();

    const allTime = document.getElementsByClassName("plyr__time plyr__time--current")[0].innerHTML;
    const allTimeInSeconds = (parseInt(allTime.split(":")[0]) * 60 ) + parseInt(allTime.split(":")[1]);

    const timePlayed = setInterval(() => {
        json_data = {
            "currentTime": player.currentTime
        }
        localStorage.setItem(song_name, JSON.stringify(json_data));
    }, 1000);
});
