import {defaultTheme, defineUserConfig} from 'vuepress'

export default defineUserConfig({
    // 将 base 设置为 "/<REPO>/"
    base : '/learn_rust_web/',
    lang: 'zh-CN',
    title: 'rust Web 的 渐进式教程',
    description: 'rust Web 的 渐进式教程',
    theme: defaultTheme({
        // 在这里进行配置
        navbar: [
            // NavbarItem
            {
                text: 'RustWeb的渐进式教程',
                link: '/rust/',
            },
            // NavbarGroup
            {
                text: '联系方式',
                children: [
                    'https://baidu.com',
                ],
            },
        ],
        // 侧边栏对象
        // 不同子路径下的页面会使用不同的侧边栏
        sidebar: {
            '/rust/': [
                {
                    text : 'RustWeb的渐进式教程',
                    children : [
                        {
                            text : '1_初始化项目.md',
                            link : '/rust/1_初始化项目.md'
                        }
                    ]
                }
            ]
        }
    }),
})