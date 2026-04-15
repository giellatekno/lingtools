import type { ParadigmResponseType } from "./types";

interface AnalyzeJSON {
    parsed: AnalyzeItem[];
    raw: string;
}

interface AnalyzeItem {
    lemma: string;
    pos: string;
    tags: string[];
    wordform: string;
}

export function analyze_parser(data: AnalyzeJSON | undefined) {
    if (!data) return;

    // Groups results by wordform
    let grouped = Object.groupBy(data.parsed, ({ wordform }) => wordform);
    return Object.entries(grouped).map(([key, items]) => ({
        key: key,
        items: items as AnalyzeItem[],
    }));
}

export interface ParsedAnalysis {
    wordform: string;
    analyses: AnalysisElement[][];
}

interface AnalysisElement {
    lemma: string;
    verbtype: string;
    tags: string;
    syntax: string;
    relation?: string;
}

function splitStringsByTab(strings: string[]): string[][] {
    const result: string[][] = [];
    let currentGroup: string[] = [];
    for (const str of strings) {
        if (str.startsWith("\t") && !str.startsWith("\t\t")) {
            if (currentGroup.length > 0) {
                result.push(currentGroup);
            }
            currentGroup = [str];
        } else {
            currentGroup.push(str);
        }
    }
    if (currentGroup.length > 0) {
        result.push(currentGroup);
    }
    return result;
}

export function disambiguate_parser(data: string) {
    console.log(data);
    const results: ParsedAnalysis[] = [];

    const analyses = data.split("\n:").map((a) => a.trim());
    console.log(analyses);

    const analysis_re = new RegExp(/"([^\s]+)" (<[^\s]+>)?(?: )?([^@#]+) (@[^\s]+)?/);
    const wordform_re = new RegExp(/"<([^\s]+)>"/);

    for (const analysis of analyses) {
        if (analysis === "\\n") continue;
        const analysis_lines = analysis.split("\n");
        const wordform = wordform_re.exec(analysis_lines[0]);
        let parsed: ParsedAnalysis = {
            wordform: wordform ? wordform[1] : "",
            analyses: [],
        };

        const analysis_groups = splitStringsByTab(analysis_lines.slice(1));

        for (const analysis_group of analysis_groups) {
            const group = [];
            for (const elem of analysis_group) {
                const analysis_parts = analysis_re.exec(elem);
                if (!analysis_parts) {
                    console.log("Skipping:", elem);
                    continue;
                }
                const obj: AnalysisElement = {
                    lemma: analysis_parts[1] ?? "",
                    verbtype: analysis_parts[2] ?? "",
                    tags: analysis_parts[3] ?? "",
                    syntax: analysis_parts[4] ?? "",
                };
                group.push(obj);
            }
            parsed.analyses.push(group);
        }
        results.push(parsed);
    }
    console.log(results);
    return results;
}

export function dependency_parser(data: string) {
    const results: ParsedAnalysis[] = [];

    const analyses = data.split("\n:").map((a) => a.trim());

    const analysis_re = new RegExp(
        /"([^\s]+)" (<[^\s]+>)?(?: )?([^@#]+) (@[^\s]+)?(?: )?(#[^\s]+)/,
    );
    const wordform_re = new RegExp(/"<([^\s]+)>"/);

    for (const analysis of analyses) {
        if (analysis === "\\n") continue;
        const analysis_lines = analysis.split("\n");
        const wordform = wordform_re.exec(analysis_lines[0]);
        // console.log(wordform);
        let parsed: ParsedAnalysis = {
            wordform: wordform ? wordform[1] : "",
            analyses: [],
        };

        const analysis_groups = splitStringsByTab(analysis_lines.slice(1));
        // console.log(analysis_groups);

        for (const analysis_group of analysis_groups) {
            const group = [];
            for (const elem of analysis_group) {
                // console.log(elem);
                const analysis_parts = analysis_re.exec(elem);
                if (!analysis_parts) {
                    console.log("Skipping:", elem);
                    continue;
                }
                const obj: AnalysisElement = {
                    lemma: analysis_parts[1] ?? "",
                    verbtype: analysis_parts[2] ?? "",
                    tags: analysis_parts[3] ?? "",
                    syntax: analysis_parts[4] ?? "",
                    relation: analysis_parts[5] ?? "",
                };
                group.push(obj);
            }
            parsed.analyses.push(group);
        }
        results.push(parsed);
    }
    console.log(results);
    return results;
}

export function generate_parser(data: string) {
    console.log(data);
    const parsed = data
        .split("\n")
        .filter((line) => line.length > 0)
        .map((line) => line.split("\t"))
        .filter((splits) => splits[2] !== "inf")
        .map((splits) => splits[1]);
    console.log(parsed);
    return parsed;
}

export function hyphenate_parser(data: string) {
    const results = new Map();
    const lines = data.trim().split("\n");

    for (const line of lines) {
        const trimmed_line = line.trim();

        if (trimmed_line.length > 0) {
            const parts = trimmed_line.split("\t");

            const input_word = parts[0].replace("-e ", "");
            const hyphenated_word = parts[1].replace("-e ", "");
            const score = parseFloat(parts[2]);

            const variation = {
                hyphenated_word: hyphenated_word,
                score: score,
            };

            if (!results.has(input_word)) {
                results.set(input_word, {
                    input_word: input_word,
                    variations: [],
                });
            }

            const entry = results.get(input_word);

            entry.variations.push(variation);
        }
    }
    return Array.from(results.values());
}

interface ParadigmItem {
    lemma: string;
    pos: string;
    tags: string[];
    wordform: string;
    weight?: number;
}

interface ParadigmApiResult {
    results: ParadigmItem[][];
    other_forms?: ParadigmItem[];
}

export interface ParsedParadigm {
    lemma: string;
    pos: string;
    subclass: string;
    wordforms: Map<string, Set<string>>;
}

interface ParsedResult {
    paradigms: ParsedParadigm[];
    other_hits: {
        lemma: string;
        pos: string;
        subclass: string;
    }[];
}

export function paradigm_parser(data: ParadigmResponseType): ParsedResult {
    console.log(data);
    const paradigms: ParsedParadigm[] = data.paradigm_forms.map((entry) => ({
        lemma: entry.lemma,
        pos: entry.pos,
        subclass: entry.subclass ?? "",
        wordforms: new Map(entry.forms.map(({ tags, forms }) => [tags, new Set(forms)])),
    }));

    const other_hits = data.other_forms.map((entry) => ({
        lemma: entry.lemma,
        pos: entry.pos,
        subclass: entry.subclass ?? "",
    }));

    return { paradigms, other_hits };
}

export function transcribe_parser(data: string) {
    console.log(data);
    const parsed = data
        .split("\n")
        .filter((line) => line.length > 0)
        .map((line) => line.split("\t"))
        .filter((splits) => splits[2] !== "inf")
        .map((splits) => splits[1]);
    console.log(parsed);
    return parsed;
}
