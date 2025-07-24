type CommandCallback = (...args: any[]) => any;
export interface IRsvim {
    createCommand(name: string, command: CommandCallback): void;
}
export declare class Rsvim implements IRsvim {
    readonly opt: RsvimOpt;
    createCommand(name: string, command: CommandCallback): void;
}
export declare class RsvimOpt {
    get wrap(): boolean;
    set wrap(value: boolean);
    get lineBreak(): boolean;
    set lineBreak(value: boolean);
}
export {};
