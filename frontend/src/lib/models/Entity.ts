export class Entity {
    readonly entity_id: string | undefined;
    readonly state: string | undefined;
    readonly area: string | undefined;
    readonly timestamp: string | undefined;

    constructor(data: Partial<Entity>) {
        this.entity_id = data.entity_id;
        this.state = data.state;
        this.area = data.area;
        this.timestamp = data.timestamp;
    }

    public meow() {
        return "meow"
    }

    public unique_id(): string {
        return `${this.area}.${this.entity_id}`
    }
}