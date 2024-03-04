import {Entity} from "$lib/models/Entity";

export async function load({fetch, params}): Promise<{ entities: Entity[] }> {
    const response = await fetch(`http://0.0.0.0:8080/api/enity/${params.area}/${params.entity_id}`);
    const data = await response.json();

    // Map over the data to create new instances of Entity
    const entities = data.map((item: any) => new Entity(item));

    return {entities};
}
