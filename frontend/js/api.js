const API_BASE = "http://localhost:8000/api";

const api = {
    token: null,

    setToken(t) {
        this.token = t;
        localStorage.setItem("token", t);
    },

    getToken() {
        if (!this.token) {
            this.token = localStorage.getItem("token");
        }
        return this.token;
    },

    clearToken() {
        this.token = null;
        localStorage.removeItem("token");
        localStorage.removeItem("user");
    },

    async request(method, path, body) {
        const headers = { "Content-Type": "application/json" };
        const t = this.getToken();
        if (t) headers["Authorization"] = "Bearer " + t;

        const opts = { method, headers };
        if (body) opts.body = JSON.stringify(body);

        try {
            const res = await fetch(API_BASE + path, opts);
            const data = await res.json();
            return data;
        } catch (e) {
            return { success: false, error: e.message };
        }
    },

    get(path) { return this.request("GET", path); },
    post(path, body) { return this.request("POST", path, body); },
    put(path, body) { return this.request("PUT", path, body); },
    del(path) { return this.request("DELETE", path); },
};