<template>
    <el-container class="container-wrapper">
        <el-container>
            <el-aside width="150px">
                <el-menu
                    unique-opened
                    :default-active="activeTabName"
                    active-text-color="#ffd04b"
                    background-color="#303133"
                    class="el-menu-vertical-demo"
                    text-color="#fff"
                    @select="changeTab"
                >
                    <template v-for="(item, index) in menusList">
                        <el-sub-menu
                            v-if="item.children"
                            :key="index"
                            :index="index + ''"
                        >
                            <template #title>
                                <el-icon>
                                    <component :is="item.icon"></component>
                                </el-icon>
                                <span>{{ item.label }}</span>
                            </template>
                            <el-menu-item
                                v-for="(child, chi) in item.children"
                                :index="child.name"
                                :key="chi"
                            >
                                <span>{{ child.label }}</span>
                            </el-menu-item>
                        </el-sub-menu>

                        <el-menu-item
                            v-else
                            :key="item.name"
                            :index="item.name"
                        >
                            <el-icon>
                                <component :is="item.icon"></component>
                            </el-icon>
                            <span>{{ item.label }}</span>
                        </el-menu-item>
                    </template>
                </el-menu>
            </el-aside>
            <el-main>
                <router-view />
            </el-main>
        </el-container>
    </el-container>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { useRouter, useRoute } from "vue-router";
import { Comment, Histogram, WalletFilled } from "@element-plus/icons-vue";

// 定义组件
const components = {
    Comment,
    Histogram,
    WalletFilled,
};

// 菜单列表数据
const menusList = ref([
    {
        name: "demo",
        label: "demo",
        icon: "WalletFilled",
    },
    // {
    //     icon: "WalletFilled",
    //     label: "demo",
    //     children: [
    //         {
    //             name: "one",
    //             label: "one",
    //         },
    //         {
    //             name: "two",
    //             label: "two",
    //         },
    //     ],
    // },
]);

// 路由和选项卡状态
const router = useRouter();
const route = useRoute();
const activeTabName = ref("");

// 激活选项卡时触发的回调函数
const changeTab = (name) => {
    activeTabName.value = name;
    router.push({
        name: name,
    });
};

// 页面加载时设定当前激活的选项卡
onMounted(() => {
    activeTabName.value = route.name;
});
</script>

<style scoped>
.container-wrapper {
    width: 100%;
    height: 100%;
    padding: 0;
    margin: 0;
}
.el-main {
    height: 100%;
    width: 100%;
    padding: 30px;
    margin: 0;
}
.el-aside {
    height: 100%;
    background-color: #303133;
}
.el-menu-item {
    display: flex;
    justify-content: flex-start;
    align-items: center;
}
.el-submenu {
    width: 200px;
}
</style>
