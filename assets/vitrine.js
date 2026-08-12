/* vitrine runtime — <md-section> transclusion + response forms.
 * Appended after the vendored commonmark.js (window.commonmark).
 * Contract shared with the Rust side (extract.rs / render.rs):
 * plain CommonMark, front matter stripped, ATX headings only,
 * fence-aware, duplicate slugs suffixed -1, -2, … */
(() => {
	const stripFrontMatter = (md) => {
		if (!md.startsWith("---\n")) return md;
		const end = md.indexOf("\n---\n", 4);
		return end === -1 ? md : md.slice(end + 5);
	};

	const slugify = (heading) => {
		let out = "";
		let lastHyphen = true;
		for (const c of heading.trim()) {
			if (/\p{L}|\p{N}/u.test(c)) {
				out += c.toLowerCase();
				lastHyphen = false;
			} else if (!lastHyphen) {
				out += "-";
				lastHyphen = true;
			}
		}
		return out.replace(/-+$/, "");
	};

	const extract = (md, anchor) => {
		const lines = stripFrontMatter(md).split("\n");
		const heads = [];
		const seen = new Map();
		let fence = null;
		lines.forEach((line, i) => {
			const t = line.trimStart();
			if (fence) {
				if (t.startsWith(fence)) fence = null;
				return;
			}
			if (t.startsWith("```") || t.startsWith("~~~")) {
				fence = t.slice(0, 3);
				return;
			}
			const m = /^(#{1,6})(\s+.*|)$/.exec(t);
			if (!m) return;
			const title = m[2].trim().replace(/[# ]+$/, "");
			const base = slugify(title);
			const n = seen.get(base) || 0;
			seen.set(base, n + 1);
			heads.push({
				i,
				level: m[1].length,
				slug: n === 0 ? base : `${base}-${n}`,
			});
		});
		const pos = heads.findIndex((h) => h.slug === anchor);
		if (pos === -1) return null;
		const { i: start, level } = heads[pos];
		const next = heads.slice(pos + 1).find((h) => h.level <= level);
		const end = next ? next.i : lines.length;
		return lines.slice(start, end).join("\n").replace(/\s+$/, "") + "\n";
	};

	const renderMd = (md) => {
		const reader = new commonmark.Parser();
		const writer = new commonmark.HtmlRenderer();
		return writer.render(reader.parse(md));
	};

	class MdSection extends HTMLElement {
		async connectedCallback() {
			if (!/^https?:$/.test(location.protocol)) return; // file://: baked copy stands
			const ref = this.getAttribute("ref");
			if (!ref) return;
			const [path, anchor] = ref.split("#");
			try {
				const res = await fetch(path, { cache: "no-cache" });
				if (!res.ok) throw new Error(res.status);
				const md = await res.text();
				const span = anchor
					? extract(md, anchor)
					: stripFrontMatter(md).replace(/\s+$/, "") + "\n";
				if (span == null) throw new Error(`anchor ${anchor} not found`);
				this.innerHTML = renderMd(span);
				this.setAttribute("data-live", "");
			} catch (err) {
				this.setAttribute("data-unresolved", String(err));
			}
		}
	}
	customElements.define("md-section", MdSection);

	/* Forms marked data-respond POST their fields as JSON to the inbox. */
	const slug = () => {
		const m = /\/\.vitrine\/([a-z0-9-]+)\//.exec(location.pathname);
		return m ? m[1] : null;
	};

	document.addEventListener("submit", async (e) => {
		const form = e.target;
		if (
			!(form instanceof HTMLFormElement) ||
			!form.hasAttribute("data-respond")
		)
			return;
		e.preventDefault();
		const status =
			form.querySelector("[data-respond-status]") ||
			form.appendChild(Object.assign(document.createElement("p"), {}));
		status.setAttribute("data-respond-status", "");
		const s = slug();
		if (!s) {
			status.textContent =
				"not served from a .vitrine artifact path; cannot submit";
			return;
		}
		const payload = Object.fromEntries(new FormData(form).entries());
		try {
			const res = await fetch(`/respond/${s}`, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify(payload),
			});
			if (!res.ok) throw new Error(await res.text());
			status.textContent = "response recorded";
		} catch (err) {
			status.textContent = `submit failed: ${err} (is vitrine serve running?)`;
		}
	});
})();
