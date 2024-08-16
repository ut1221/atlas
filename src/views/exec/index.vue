<template>
    <el-table :data="tableData" :height="tableMaxHeight" style="width: 100%">
        <el-table-column
            prop="date"
            label="Date"
            width="180"
            show-overflow-tooltip
        />
        <el-table-column
            prop="name"
            label="Name"
            width="180"
            show-overflow-tooltip
        />
        <el-table-column prop="address" label="Address" show-overflow-tooltip />
        <el-table-column fixed="right" label="Operations" min-width="120">
            <template #default="scope">
                <el-button
                    link
                    type="primary"
                    size="small"
                    @click="runOperation(scope.row)"
                >
                    运行
                </el-button>
            </template>
        </el-table-column>
    </el-table>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api";

const tableData = ref([
    {
        date: "2016-05-03",
        name: "Tom",
        address: "No. 189, Grove St, Los Angeles",
    },
]);

const runOperation = async (row) => {
    try {
        const newData = await invoke("update_data", { row });
        tableData.value = newData;
    } catch (error) {
        console.error("Failed to update data:", error);
    }
};

const tableMaxHeight = ref(0);
const setTableMaxHeight = () => {
    const screenHeight = window.innerHeight;
    tableMaxHeight.value = screenHeight * 0.84; // 设置表格高度为屏幕高度的60%
};
onMounted(() => {
    setTableMaxHeight();
    window.addEventListener("resize", setTableMaxHeight);
});
onBeforeUnmount(() => {
    window.removeEventListener("resize", setTableMaxHeight);
});
</script>
