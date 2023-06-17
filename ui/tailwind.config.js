/** @type {import('tailwindcss').Config} */
// 由于 tailwindcss 无法识别全局安装路径，所以需要手动指定
// 本次tailwindcss 为pnpm全局安装
const daisyui = require("/Users/zhangchi/Library/pnpm/global/5/node_modules/daisyui");
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {},
    },
    plugins: [daisyui],
}