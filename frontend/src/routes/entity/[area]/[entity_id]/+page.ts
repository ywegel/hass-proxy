import {Entity} from "$lib/models/Entity";

class EntityWithData {
    public entity_id: string | undefined;
    public area: string | undefined;
    public data: EntityData[] | undefined;

    constructor(data: Partial<EntityWithData>) {
        this.entity_id = data.entity_id;
        this.area = data.area;
        this.data = data.data;
    }
}

class EntityData {
    state: string | undefined;
    timestamp: string | undefined;
}

export async function load({fetch, params}): Promise<{ entityWithData: EntityWithData }> {
    const response = await fetch(`http://localhost:8080/api/entity/${params.area}/${params.entity_id}`);
    const data = await response.json();

    const entityWithData =  new EntityWithData(data);

    return {entityWithData};
}
