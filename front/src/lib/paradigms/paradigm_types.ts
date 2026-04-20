import type { ParsedParadigm } from "$lib/parsers";

export interface JsonRow {
    label: string;
    tags: string[];
    colspan?: number;
    prefixes?: string[];
    separator?: boolean;
}

export interface JsonTable {
    tId?: string;
    title?: string;
    headers: string[];
    rows: JsonRow[];
    showIf?: string[]; // Tags that must be present
}

export interface JsonSection {
    sId?: string;
    title?: string;
    tables: JsonTable[];
    validateRows?: boolean;
    showIf?: string[]; // Tags that must be present
}

export interface JsonLanguageSchema {
    sections: JsonSection[];
}

export interface Row {
    label: string | (() => string);
    tags: string[];
    colspan?: number;
    prefixes?: string[];
    separator?: boolean;
}

export interface Table {
    tId?: string;
    title?: () => string;
    headers: (() => string)[];
    rows: Row[];
    showIf?: (elem: ParsedParadigm) => boolean;
}

export interface Section {
    sId?: string;
    title?: () => string;
    tables: Table[];
    validateRows?: boolean;
    showIf?: (elem: ParsedParadigm) => boolean;
}

export interface LanguageSchema {
    sections: Section[];
}
