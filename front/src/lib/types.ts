import * as z from "zod";

export const AnalyzeResponse = z.object({
    results: z.array(
        z.object({
            analyses: z.array(z.string()),
            wordform: z.string(),
        }),
    ),
});
export type AnalyzeResponseType = z.infer<typeof AnalyzeResponse>;

// export const DependencyResponse = z.object();
// export const DisambiguateResponse = z.object();

export const GenerateResponse = z.object({
    results: z.array(
        z.object({
            analysis: z.string(),
            wordforms: z.array(z.string()),
        }),
    ),
});
export type GenerateResponseType = z.infer<typeof GenerateResponse>;
// export const HyphenateResponse = z.object();
// export const NumbersResponse = z.object();

export const ParadigmResponse = z.object({
    input: z.tuple([z.string(), z.nullable(z.string())]),
    paradigm_forms: z.array(
        z.object({
            lemma: z.string(),
            pos: z.string(),
            subclass: z.nullable(z.string()),
            forms: z.array(
                z.object({
                    forms: z.array(z.string()),
                    tags: z.string(),
                }),
            ),
        }),
    ),
    other_forms: z.array(
        z.object({
            lemma: z.string(),
            pos: z.string(),
            subclass: z.nullable(z.string()),
        }),
    ),
});
export type ParadigmResponseType = z.infer<typeof ParadigmResponse>;

// export const TranscribeResponse = z.object();
