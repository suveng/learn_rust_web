import {defaultTheme, defineUserConfig} from 'vuepress'

export default defineUserConfig({
    // 将 base 设置为 "/<REPO>/"
    base : '/learn_rust_web/',
    lang: 'zh-CN',
    title: 'RustWeb的渐进式教程',
    description: 'RustWeb的渐进式教程',
    theme: defaultTheme({
        // 在这里进行配置
        navbar: [
            // NavbarItem
            // {
            //     text: 'RustWeb的渐进式教程',
            //     link: '/rust/',
            // },
            // NavbarGroup
            {
                text: '寻求rust开发工作',
                link: '/resume/resume.md',
            },
        ],
        // 侧边栏对象
        // 不同子路径下的页面会使用不同的侧边栏
        sidebar: {
            '/': [
                {
                    text : 'RustWeb的渐进式教程',
                    children : [
                        {
                            text : '1_初始化项目.md',
                            link : '/rust/1_初始化项目.md'
                        },
                        {
                            text : '2_编写简单的grpc接口.md',
                            link : '/rust/2_编写简单的grpc接口.md'
                        },

                    ]
                }
            ]
        }
    }),
})