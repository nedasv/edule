// Show API response in the result box
function showResult(data) {
    const box = document.getElementById("result-box");
    const output = document.getElementById("result-output");
    box.style.display = "block";
    output.textContent = JSON.stringify(data, null, 2);
}

// Login handler
document.getElementById("login-btn").addEventListener("click", async () => {
    const username = document.getElementById("username").value;
    const password = document.getElementById("password").value;
    const errEl = document.getElementById("login-error");
    errEl.textContent = "";

    const res = await api.post("/auth/login", { username, password });

    if (res.success && res.data && res.data.token) {
        api.setToken(res.data.token);
        localStorage.setItem("user", JSON.stringify(res.data));
        showDashboard(res.data);
    } else {
        errEl.textContent = res.error || "Login failed";
    }
});

// Logout
document.getElementById("logout-btn").addEventListener("click", () => {
    api.clearToken();
    document.getElementById("login-page").style.display = "block";
    document.getElementById("dashboard").style.display = "none";
    document.getElementById("result-box").style.display = "none";
});

// Show dashboard after login
function showDashboard(userData) {
    document.getElementById("login-page").style.display = "none";
    document.getElementById("dashboard").style.display = "block";
    document.getElementById("welcome-msg").textContent =
        "Logged in as: " + userData.user.username + " (" + userData.user.role + ")";
    buildNav(userData.user.role);
}

function buildNav(role) {
    document.getElementById("nav-bar").innerHTML = "<em>Navigation will appear here</em>";
    document.getElementById("content").innerHTML = "<p>Select a section from the nav bar above.</p>";
}

// Auto-login if token exists
(function checkSession() {
    const saved = localStorage.getItem("user");
    if (saved) {
        try {
            const userData = JSON.parse(saved);
            api.setToken(localStorage.getItem("token"));
            showDashboard(userData);
        } catch (e) {
            api.clearToken();
        }
    }
})();