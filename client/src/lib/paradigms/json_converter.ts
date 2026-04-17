import type {
    JsonLanguageSchema,
    LanguageSchema,
    JsonSection,
    Section,
    JsonTable,
    Table,
    JsonRow,
    Row,
} from "./paradigm_types";
import { resolveLocaleKey } from "./locale_map";
import { has_tags } from "./paradigm_utils";

/**
 * Convert a JSON section to a runtime Section
 */
function convertSection(section: JsonSection): Section {
    return {
        sId: section.sId,
        title: section.title ? resolveLocaleKey(section.title) : undefined,
        tables: section.tables.map(convertTable),
        validateRows: section.validateRows,
        showIf: section.showIf ? has_tags(...section.showIf) : undefined,
    };
}

/**
 * Convert a JSON table to a runtime Table
 */
function convertTable(table: JsonTable): Table {
    return {
        tId: table.tId,
        title: table.title ? resolveLocaleKey(table.title) : undefined,
        headers: table.headers.map(resolveLocaleKey),
        rows: table.rows.map(convertRow),
        showIf: table.showIf ? has_tags(...table.showIf) : undefined,
    };
}

/**
 * Convert a JSON row to a runtime Row
 */
function convertRow(row: JsonRow): Row {
    return {
        label: resolveLocaleKey(row.label),
        tags: row.tags,
        colspan: row.colspan,
        prefixes: row.prefixes,
        separator: row.separator,
    };
}

/**
 * Convert a complete JSON language schema to runtime format
 */
export function convertJsonSchema(jsonSchema: JsonLanguageSchema): LanguageSchema {
    return {
        sections: jsonSchema.sections.map(convertSection),
    };
}
