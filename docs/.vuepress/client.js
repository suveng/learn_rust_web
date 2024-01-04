import { defineClientConfig } from "@vuepress/client";
import { defineGiscusConfig } from "vuepress-plugin-comment2/client";

import Layout from "./layouts/Layout.vue";

defineGiscusConfig({
    repo: "suveng/learn_rust_web",
    repoId: "R_kgDOK-uFeg",
    category: "Announcements",
    categoryId: "DIC_kwDOK-uFes4CcLpE",
});

export default defineClientConfig({
    layouts: {
        // we override the default layout to provide comment service
        Layout,
    },
});