<script lang="ts">
    import type {PageData} from "./$types";
    import {page} from "$app/stores";
    import Chart from "$lib/components/Chart.svelte";

    export let data: PageData;
    let entityWithData = data.entityWithData;

    let chartData;
    if(entityWithData.data?.[0].state == "off") {
        chartData = entityWithData.data?.map((d: any) => {
            if (d.state === 'on') {
                d.state = 1;
            } else if (d.state === 'off') {
                d.state = 0;
            }
            return d;
        });
    } else {
        chartData = entityWithData.data;
    }

    const option = {
        series: [{
            name: "Desktops",
            data: chartData
        }],
        chart: {
            height: 350,
            type: 'line',
            zoom: {
                enabled: false
            }
        },
        dataLabels: {
            enabled: false
        },
        stroke: {
            curve: 'straight'
        },
        labels: {
            formatter: function(val: any) {
                if (val === 1) {
                    return 'on';
                } else if (val === 0) {
                    return 'off';
                }
                return val;
            }
        }
    }

</script>

<h1>Entity: {$page.params.area}.{$page.params.entity_id} </h1>

<Chart options="{option}"/>
