<?xml version="1.0"?>
<!--+
		| Transforms texts for cgi-interface to Forrest documents according
		| to the process language and the tool
		|
		+-->

<!DOCTYPE xsl:stylesheet [
  <!ENTITY % corpusDTD SYSTEM "https://giellatekno.uit.no/dtd/corpus.dtd">
  %corpusDTD;
  <!ENTITY % canengDTD SYSTEM "https://giellatekno.uit.no/dtd/caneng.dtd">
  %canengDTD;
  <!ENTITY % canfinDTD SYSTEM "https://giellatekno.uit.no/dtd/canfin.dtd">
  %canfinDTD;
  <!ENTITY % cannobDTD SYSTEM "https://giellatekno.uit.no/dtd/cannob.dtd">
  %cannobDTD;
  <!ENTITY % canrusDTD SYSTEM "https://giellatekno.uit.no/dtd/canrus.dtd">
  %canrusDTD;
  <!ENTITY % cansmaDTD SYSTEM "https://giellatekno.uit.no/dtd/cansma.dtd">
  %cansmaDTD;
  <!ENTITY % cansmeDTD SYSTEM "https://giellatekno.uit.no/dtd/cansme.dtd">
  %cansmeDTD;
  <!ENTITY % cansmjDTD SYSTEM "https://giellatekno.uit.no/dtd/cansmj.dtd">
  %cansmjDTD;
]>

<xsl:stylesheet
    xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
    xmlns:i18n="http://apache.org/cocoon/i18n/2.1"
    version="1.0">

  <xsl:param name="lang"/>
  <xsl:param name="pagelang"/>
  <xsl:param name="tool"/>

  <xsl:variable name="show_each_dict" select="false()"/>

  <!-- lang declarations -->
  <xsl:variable name="all_langs" select="concat(
					 '_bxr_ciw_cor_',
					 '_xxx_est_evn_',
					 '_fao_fin_gle_',
					 '_fkv_hdn_ipk_',
					 '_izh_kal_kca_',
					 '_kpv_liv_mdf_',
					 '_koi_fit_xxx_',
					 '_mhr_mns_mrj_',
					 '_myv_nio_nob_',
					 '_olo_rmf_xxx_',
					 '_xxx_rus_som_',
					 '_sjd_sje_sma_',
					 '_sme_smj_smn_',
					 '_sms_udm_vep_',
					 '_vot_vro_yrk_'
					 )"/>

  <xsl:variable name="analysis_langs" select="$all_langs"/>

  <xsl:variable name="paradigm_langs" select="concat(
					      '_bxr_ciw_cor_',
					      '_fit_xxx_evn_',
					      '_koi_fao_fin_',
					      '_fkv_gle_ipk_',
					      '_izh_kal_kca_',
					      '_kpv_liv_mdf_',
					      '_mhr_mns_mrj_',
					      '_myv_nio_nob_',
					      '_olo_rus_som_',
					      '_sjd_sje_sma_',
					      '_sme_smj_smn_',
					      '_sms_udm_vep_',
					      '_vot_vro_yrk_'
					      )"/>

  <xsl:variable name="generation_langs" select="concat(
						'_bxr_ciw_cor_',
						'_xxx_est_evn_',
						'_fao_fin_koi_',
						'_gle_hdn_ipk_',
						'_izh_kal_kca_',
						'_kpv_liv_mdf_',
						'_mhr_mns_mrj_',
						'_myv_nio_nob_',
						'_olo_rus_som_',
						'_sjd_sje_sma_',
						'_sme_smj_smn_',
						'_sms_udm_vep_',
						'_vot_vro_yrk_'
						)"/>

  <xsl:variable name="num_langs" select="concat(
					 '_xxx_xxx_xxx_',
					 '_xxx_xxx_fin_',
					 '_xxx_hdn_xxx_',
					 '_xxx_xxx_xxx_',
					 '_xxx_liv_mdf_',
					 '_mhr_xxx_myv_',
					 '_xxx_olo_rus_',
					 '_sjd_xxx_sma_',
					 '_sme_smj_smn_',
					 '_sms_xxx_xxx_',
					 '_yrk_xxx_xxx_'
					 )"/>

  <!-- this is the no_num_langs list: kept for debugging purposes -->
  <!-- '_bxr_ciw_cor_', -->
  <!-- '_fao_fkv_hdn_', -->
  <!-- '_izh_ipk_kal_', -->
  <!-- '_kca_kpv_mrj_', -->
  <!-- '_nio_sje_udm_', -->
  <!-- '_vep_koi__' -->

  <xsl:variable name="dict_langs" select="concat(
					      '_xxx_xxx_fin_',
					      '_fkv_hdn_izh_',
					      '_kpv_liv_koi_',
					      '_xxx_xxx_nob_',
					      '_olo_xxx_sjt_',
					      '_sjd_sje_sma_',
					      '_sme_smj_smn_',
					      '_sms_udm_vep_',
					      '_vot_vro_xxx_'
					      )"/>

  <xsl:variable name="mordict_langs" select="concat(
					     '_mdf_myv_',
					     '_____'
					      )"/>

  <xsl:variable name="maridict_langs" select="concat(
					     '_mhr_mrj_',
					     '_____'
					      )"/>


    <xsl:variable name="vadadict_langs" select="concat(
					     '_yrk_mns_',
					     '_____'
					      )"/>


  <xsl:variable name="geo_langs"  select="concat(
					 '_sma_sme_smj_',
					 '____'
					 )"/>

  <xsl:variable name="risten_langs"  select="concat(
					     '_sma_sme_smj_',
					     '____'
					     )"/>

  <xsl:variable name="webdict_langs" select="concat(
					     '_fkv_mdf_myv_',
					     '_kpv_sjd_sma_',
					     '_sme_smj_sms_',
					     '____'
					     )"/>

  <xsl:variable name="vd_langs" select="concat(
					'_sme_sma_fkv_',
					'____'
					)"/>

  <!-- page declarations -->
  <xsl:variable name="dpage"	select="concat('d-', $lang, '.', $pagelang, '.html')"/>
  <xsl:variable name="ppage"	select="concat('p-', $lang, '.', $pagelang, '.html')"/>
  <xsl:variable name="gpage"	select="concat('g-', $lang, '.', $pagelang, '.html')"/>
  <xsl:variable name="numpage" select="concat('/num', '.', $pagelang, '.html')"/>
  <!-- npage to replace the numpage: todo -->
  <!--xsl:variable name="npage"	select="concat('n-', $lang, '.', $pagelang, '.html')"/-->

  <xsl:variable name="webdict_page">
    <xsl:if test="$lang='smj'">
      <xsl:value-of select="'ak/smj2nob/'"/>
    </xsl:if>
  </xsl:variable>

  <!-- add lang_code for existing URLs or lang_code:url
  for a new dictionary URL -->
  <xsl:variable name="nds_page">
    <xsl:if test="contains('_fkv_olo_vep_', $lang)">
      <xsl:value-of select="'sanat'"/>
    </xsl:if>
    <xsl:if test="contains('_hdn_', $lang)">
      <xsl:value-of select="'guusaaw'"/>
    </xsl:if>
    <xsl:if test="contains('_izh_liv_vro_', $lang)">
      <xsl:value-of select="'sonad'"/>
    </xsl:if>
    <xsl:if test="contains('_kpv_udm_koi_', $lang)">
      <xsl:value-of select="'kyv'"/>
    </xsl:if>
    <xsl:if test="contains('_mdf_myv_', $lang)">
      <xsl:value-of select="'valks'"/>
    </xsl:if>
    <xsl:if test="contains('_mrh_mrj_', $lang)">
      <xsl:value-of select="'muter'"/>
    </xsl:if>
    <xsl:if test="contains('_sjd_', $lang)">
      <xsl:value-of select="'sanj'"/>
    </xsl:if>
	<xsl:if test="contains('_sma_', $lang)">
      <xsl:value-of select="'baakoeh'"/>
    </xsl:if>
    <xsl:if test="contains('_sme_', $lang)">
      <xsl:value-of select="'sanit'"/>
    </xsl:if>
    <xsl:if test="contains('_smn_', $lang)">
      <xsl:value-of select="'saanih'"/>
    </xsl:if>
	<xsl:if test="contains('_sms_', $lang)">
      <xsl:value-of select="'saan'"/>
    </xsl:if>
    <xsl:if test="contains('_yrk_', $lang)">
      <xsl:value-of select="'vada'"/>
    </xsl:if>
  </xsl:variable>

  <xsl:variable name="grammar_page">
    <xsl:if test="$lang='sme' and contains('_nob_sma_smj_rus_', concat('_', $pagelang, '_'))">
      <xsl:value-of select="'grammatihkka.nob'"/>
    </xsl:if>
    <xsl:if test="$lang='sme' and contains('_sme_fin_', concat('_', $pagelang, '_'))">
      <xsl:value-of select="'grammatihkka'"/>
    </xsl:if>
    <xsl:if test="$lang='sme' and contains('_eng_', concat('_', $pagelang, '_'))">
      <xsl:value-of select="'grammatihkka.eng'"/>
    </xsl:if>
    <xsl:if test="$lang='sme' and contains('_rus_', concat('_', $pagelang, '_'))">
      <xsl:value-of select="'grammatihkka.eng'"/>
    </xsl:if>
    <xsl:if test="$lang='sma'">
      <xsl:value-of select="'grammatikk.nob'"/>
    </xsl:if>
  </xsl:variable>

  <xsl:variable name="korp_page">
    <xsl:if test="contains('_sme_sma_smj_sjd_smn_', concat('_', $lang, '_'))">
      <xsl:value-of select="'korp'"/>
    </xsl:if>
    <xsl:if test="contains('_fao_fit_fkv_olo_vep_vro_', concat('_', $lang, '_'))">
      <xsl:value-of select="'f_korp'"/>
    </xsl:if>
    <xsl:if test="contains('_kpv_koi_mdf_mhr_mrj_myv_udm_', concat('_', $lang, '_'))">
      <xsl:value-of select="'u_korp'"/>
    </xsl:if>
  </xsl:variable>

  <xsl:variable name="keyboard_page">
    <xsl:if test="contains('_sjd_sje_sma_sme_smj_smn_sms__', $lang) and contains('_eng_', concat('_', $pagelang, '_'))">
      <xsl:value-of select="'https://divvun.no/en/keyboards/index.html'"/>
    </xsl:if>
    <xsl:if test="contains('_sjd_sje_sma_sme_smj_smn_sms__', $lang) and contains('_fin_', concat('_', $pagelang, '_'))">
      <xsl:value-of select="'https://divvun.no/fi/keyboards/index.html'"/>
    </xsl:if>
    <xsl:if test="contains('_sjd_sje_sma_sme_smj_smn_sms__', $lang) and contains('_nob_', concat('_', $pagelang, '_'))">
      <xsl:value-of select="'https://divvun.no/no/keyboards/index.html'"/>
    </xsl:if>
    <xsl:if test="contains('_sjd_sje_sma_sme_smj_smn_sms__', $lang) and contains('_rus_', concat('_', $pagelang, '_'))">
      <xsl:value-of select="'https://divvun.no/en/keyboards/index.html'"/>
    </xsl:if>
    <xsl:if test="contains('_sjd_sje_sma_sme_smj_smn_sms__', $lang) and contains('_sme_', concat('_', $pagelang, '_'))">
      <xsl:value-of select="'https://divvun.no/keyboards/index.html'"/>
    </xsl:if>
  </xsl:variable>

  <xsl:variable name="risten_page">
    <xsl:choose>
  <!-- changed from $pagelang (option 2) to $lang (option 1) -->
      <xsl:when test="$lang='sma'">
	<xsl:value-of select="'https://baakoe.org'"/>
      </xsl:when>
      <xsl:when test="$lang='sme'">
	<xsl:value-of select="'https://satni.org'"/>
      </xsl:when>
      <xsl:when test="$lang='smj'">
	<xsl:value-of select="'https://bahko.org'"/>
      </xsl:when>
<!-- Thus, the choice of satni version is dependent upon lang.
     Another option is to let it be dependent upon pagelang, and
     have nob as fallback for other pagelangs -->
<!-- option 1 (current): lang=sma pagelang=eng ==> baakoe (localised in south saami)
     option 2 (alternative): lang=sma pagelang=eng ==> satni (localised in norw) -->
<!--      <xsl:otherwise>
	<xsl:value-of select="'https://satni.org'"/>
      </xsl:otherwise> -->
    </xsl:choose>
  </xsl:variable>

  <xsl:variable name="divvun_page">
    <xsl:if test="$lang='sma'">
      <xsl:value-of select="'https://divvun.no/sma/index.html'"/>
    </xsl:if>
    <xsl:if test="$lang='sme'">
      <xsl:value-of select="'https://divvun.no'"/>
    </xsl:if>
    <xsl:if test="$lang='smj'">
      <xsl:value-of select="'https://divvun.no/smj/index.html'"/>
    </xsl:if>
  </xsl:variable>

  <xsl:variable name="oahpa_page">
    <xsl:if test="$lang='fkv'">
      <xsl:value-of select="'https://oahpa.no/kveeni/'"/>
    </xsl:if>
    <xsl:if test="$lang='myv'">
      <xsl:value-of select="'https://oahpa.no/erzya/'"/>
    </xsl:if>
    <xsl:if test="$lang='liv'">
      <xsl:value-of select="'https://testing.oahpa.no/livokel/'"/>
    </xsl:if>
    <xsl:if test="$lang='rus'">
      <xsl:value-of select="'https://testing.oahpa.no/rusoahpa/'"/>
    </xsl:if>
    <!-- <xsl:if test="$lang='sjd'">
      <xsl:value-of select="'https://oahpa.no/kiilt/'"/>
    </xsl:if> -->
    <xsl:if test="$lang='sma'">
      <xsl:value-of select="'https://oahpa.no/aarjel/'"/>
    </xsl:if>
    <xsl:if test="$lang='sme'">
      <xsl:value-of select="'https://oahpa.no/davvi/'"/>
    </xsl:if>
    <xsl:if test="$lang='smn'">
      <xsl:value-of select="'https://oahpa.no/aanaar/'"/>
    </xsl:if>
    <xsl:if test="$lang='sms'">
      <xsl:value-of select="'https://oahpa.no/nuorti/'"/>
    </xsl:if>
  </xsl:variable>

  <xsl:variable name="kursa_page">
    <xsl:if test="$lang='sme'">
      <xsl:value-of select="'https://kursa.oahpa.no/'"/>
    </xsl:if>
    <xsl:if test="$lang='sma'">
      <xsl:value-of select="'https://kuvsje.oahpa.no/'"/>
    </xsl:if>
  </xsl:variable>

  <xsl:variable name="tts_page">
    <xsl:if test="$lang='sme'">
      <xsl:value-of select="'https://divvun.no/no/tale/tale.html'"/>
    </xsl:if>
  </xsl:variable>


  <xsl:variable name="gielese_page">
    <xsl:if test="$lang='sma'">
      <xsl:value-of select="'https://gielese.no/'"/>
    </xsl:if>
  </xsl:variable>


  <xsl:variable name="visl_page">
    <xsl:if test="$lang='sme'">
      <xsl:value-of select="concat('https://beta.visl.sdu.dk/visl/', $lang)"/>
    </xsl:if>
  </xsl:variable>

  <xsl:variable name="mt_page">
    <xsl:if test="$lang='sme'">
      <xsl:value-of select="'https://jorgal.uit.no/'"/>
    </xsl:if>
  </xsl:variable>

  <xsl:variable name="mt2_page">
    <xsl:if test="contains('_sme_smj_smn_', $lang)">
      <xsl:value-of select="'https://gtweb.uit.no/mt/'"/>
    </xsl:if>
  </xsl:variable>


<xsl:template match="content">
    <document>
      <header>
	<title>
	  <!-- if no lang: use Giellatekno entity -->
	  <xsl:if test="not(lname[@lang=$lang])">
	    <xsl:value-of select="'&gte;'"/>
	  </xsl:if>
	  <!-- title pattern for nob and rus: language tools for LANG -->
	  <xsl:if test="contains('_nob_rus_', $pagelang)">
	    <xsl:value-of select="concat(lt, ' ')"/>
	  </xsl:if>
	  <xsl:value-of select="lname[@lang=$lang]"/>
	  <!-- title pattern for not(nob and rus): LANG language tools -->
	  <xsl:if test="not(contains('_nob_rus_', $pagelang))">
	    <xsl:value-of select="concat(' ' , lt)"/>
	  </xsl:if>
	</title>
      </header>
      <body>
	<p>
	  <xsl:copy-of select="introduction[@lang=$lang]"/>
	</p>
	<dl>

	  <!-- analysis -->
	  <xsl:if test="contains($analysis_langs, concat('_', $lang, '_'))">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="$dpage"/>
		</xsl:attribute>
		<xsl:value-of select="analysis"/>
	      </xsl:element>
	    </dt>
	    <dd>
	    <!-- old
	      <xsl:copy-of select="analysis_text[@lang=$lang]"/> -->
<!-- new elsewhere -->
	      <xsl:choose>
	      <!-- the following 7 lines give a model case for the elsewhere rule -->
		<xsl:when test="analysis_text[@lang=$lang]">
		  <xsl:value-of select="analysis_text[@lang=$lang]"/>
		</xsl:when>
		<xsl:otherwise>
		  <xsl:value-of select="analysis_text"/>
		</xsl:otherwise>
	      </xsl:choose>
<!-- -->
	    </dd>
	    <br/>
	  </xsl:if>

	  <!-- paradigm generation -->
	  <xsl:if test="contains($paradigm_langs, concat('_', $lang, '_'))">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="$ppage"/>
		</xsl:attribute>
			      <xsl:choose>
				<xsl:when test="paradigm[@lang=$lang]">
		<xsl:value-of select="paradigm"/>
		</xsl:when>
		<xsl:otherwise>
		  <xsl:value-of select="paradigm"/>
		</xsl:otherwise>
	      </xsl:choose>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:choose>
	      <!-- the following 7 lines give a model case for the elsewhere rule -->
		<xsl:when test="paradigm_text[@lang=$lang]">
		  <xsl:value-of select="paradigm_text[@lang=$lang]"/>
		</xsl:when>
		<xsl:otherwise>
		  <xsl:value-of select="paradigm_text"/>
		</xsl:otherwise>
	      </xsl:choose>
	    </dd>
	    <br/>
	  </xsl:if>

	  <!-- word form generation -->
	  <xsl:if test="contains($generation_langs, concat('_', $lang, '_'))">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="$gpage"/>
		</xsl:attribute>
		<xsl:value-of select="generator"/>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:value-of select="generator_text[@lang=$lang]"/>
	      </dd>
	      <br/>
	  </xsl:if>

	  <!-- number -->
	  <xsl:if test="contains($num_langs, concat('_', $lang, '_'))">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="$numpage"/>
		</xsl:attribute>
		<xsl:value-of select="numerals"/>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:choose>
		<xsl:when test="numerals_text[@lang=$lang]">
		  <xsl:value-of select="numerals_text[@lang=$lang]"/>
		</xsl:when>
		<xsl:otherwise>
		  <xsl:value-of select="numerals_text"/>
		</xsl:otherwise>
	      </xsl:choose>
	      </dd>
	      <br/>
	  </xsl:if>

	  <!-- pointer to the all-dicts page -->
          <xsl:if test="contains($dict_langs, concat('_', $lang, '_'))">
	    <dt>
	    <xsl:element name="a">
	      <xsl:attribute name="href">
		<xsl:value-of select="concat('https://dicts.uit.no/',
				      $lang, 'dicts.', $pagelang, '.html')"/>
	      </xsl:attribute>
	      <xsl:value-of select="dicts"/>
	    </xsl:element>
	  </dt>
	  <dd>
	    <xsl:value-of select="dicts_text"/>
	  </dd>
	  <br/>
	  </xsl:if>

	  	  <!-- pointer to the mordicts page -->
          <xsl:if test="contains($mordict_langs, concat('_', $lang, '_'))">
	    <dt>
	    <xsl:element name="a">
	      <xsl:attribute name="href">
		<xsl:value-of select="concat('https://dicts.uit.no/',
				      'mordicts.', $pagelang, '.html')"/>
	      </xsl:attribute>
	      <xsl:value-of select="dicts"/>
	    </xsl:element>
	  </dt>
	  <dd>
	    <xsl:value-of select="dicts_text"/>
	  </dd>
	  <br/>
	  </xsl:if>


	  	  <!-- pointer to the maridicts page -->
          <xsl:if test="contains($maridict_langs, concat('_', $lang, '_'))">
	    <dt>
	    <xsl:element name="a">
	      <xsl:attribute name="href">
		<xsl:value-of select="concat('https://dicts.uit.no/',
				      'mdicts.', $pagelang, '.html')"/>
	      </xsl:attribute>
	      <xsl:value-of select="dicts"/>
	    </xsl:element>
	  </dt>
	  <dd>
	    <xsl:value-of select="dicts_text"/>
	  </dd>
	  <br/>
	  </xsl:if>

	  	  <!-- pointer to the vadadicts page -->
          <xsl:if test="contains($vadadict_langs, concat('_', $lang, '_'))">
	    <dt>
	    <xsl:element name="a">
	      <xsl:attribute name="href">
		<xsl:value-of select="concat('https://dicts.uit.no/',
				      'yrkdicts.', $pagelang, '.html')"/>
	      </xsl:attribute>
	      <xsl:value-of select="dicts"/>
	    </xsl:element>
	  </dt>
	  <dd>
	    <xsl:value-of select="dicts_text"/>
	  </dd>
	  <br/>
	  </xsl:if>


	  <xsl:if test="$show_each_dict">
	    <!-- nds -->
	    <xsl:if test="not(normalize-space($nds_page)='')">
	      <dt>
		<xsl:element name="a">
		  <xsl:attribute name="href">
		    <xsl:value-of select="concat('https://', $nds_page, '.oahpa.no')"/>
		  </xsl:attribute>
		  <xsl:value-of select="nds"/>
		</xsl:element>
	      </dt>
	      <dd>
		<xsl:value-of select="nds_text"/>
	      </dd>
	      <br/>
	    </xsl:if>

	    <!-- webdict -->
	    <xsl:if test="contains($webdict_langs, concat('_', $lang, '_'))">
	      <dt>
		<xsl:element name="a">
		  <xsl:attribute name="href">
		    <xsl:value-of select="concat('https://gtweb.uit.no/webdict/', $webdict_page, 'index.html')"/>
		  </xsl:attribute>
		  <xsl:value-of select="webdictionaries"/>
		</xsl:element>
	      </dt>
	      <dd>
		<xsl:value-of select="webdictionaries_text"/>
	      </dd>
	      <br/>
	    </xsl:if>

	    <!-- VDdictionaries -->
	    <xsl:if test="contains($vd_langs, concat('_', $lang, '_'))">
	      <dt>
		<xsl:element name="a">
		  <xsl:attribute name="href">
		    <xsl:value-of select="concat('/words/dicts/index.dict.',
					  $lang, '.', $pagelang,'.html')"/>
		  </xsl:attribute>
		  <xsl:value-of select="VDdictionaries"/>
		</xsl:element>
	      </dt>
	      <dd>
		<xsl:value-of select="VDdictionaries_text[@lang=$lang]"/>
	      </dd>
	      <br/>
	    </xsl:if>
	  </xsl:if>

	  <!-- grammar -->
	  <xsl:if test="not(normalize-space($grammar_page)='')">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="concat('https://oahpa.no/', $lang,'/gramm/', $grammar_page, '.html')"/>
		</xsl:attribute>
		<xsl:value-of select="grammar"/>
	      </xsl:element>
	    </dt>
	    <br/>
	  </xsl:if>

	  <!-- korp -->
	  <xsl:if test="not(normalize-space($korp_page)='')">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="concat('https://gtweb.uit.no/', $korp_page)"/>
		</xsl:attribute>
		<xsl:value-of select="korp"/>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:value-of select="korp_text"/>
	    </dd>
	    <br/>
	  </xsl:if>

	  <xsl:if test="$show_each_dict">
	    <!-- geo -->
	    <xsl:if test="contains($geo_langs, concat('_', $lang, '_'))">
	      <dt>
		<xsl:element name="a">
		  <xsl:attribute name="href">
		    <xsl:value-of select="concat('geo.', $pagelang, '.html')"/>
		  </xsl:attribute>
		  <xsl:value-of select="placenames"/>
		</xsl:element>
	      </dt>
	      <dd>
		<xsl:value-of select="placenames_text"/>
	      </dd>
	      <br/>
	    </xsl:if>

	    <!-- risten -->
	    <xsl:if test="contains($risten_langs, concat('_', $lang, '_'))">
	      <dt>
		<xsl:element name="a">
		  <xsl:attribute name="href">
		    <xsl:value-of select="$risten_page"/>
		  </xsl:attribute>
		  <xsl:value-of select="risten[@lang=$lang]"/>
		</xsl:element>
	      </dt>
	      <dd>
		<xsl:value-of select="risten_text"/>
	      </dd>
	      <br/>
	    </xsl:if>
	  </xsl:if>

	  <!-- divvun -->
	  <xsl:if test="not(normalize-space($divvun_page)='')">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="$divvun_page"/>
		</xsl:attribute>
		<xsl:value-of select="divvun"/>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:value-of select="divvun_text"/>
	    </dd>
	    <br/>
	  </xsl:if>

	  <!-- keyboard -->
	  <xsl:if test="not(normalize-space($keyboard_page)='')">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="concat('https://giellalt.uit.no/infra/', $keyboard_page, '.html')"/>
		</xsl:attribute>
		<xsl:value-of select="keyboard"/>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:value-of select="keyboard_text"/>
	    </dd>
	    <br/>
	  </xsl:if>

	  <!-- oahpa -->
	  <xsl:if test="not(normalize-space($oahpa_page)='')">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="$oahpa_page"/>
		</xsl:attribute>
		<xsl:value-of select="oahpa"/>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:choose>
		<xsl:when test="oahpa_text[@lang=$lang]">
		  <xsl:value-of select="oahpa_text[@lang=$lang]"/>
		</xsl:when>
		<xsl:otherwise>
		  <xsl:value-of select="oahpa_text"/>
		</xsl:otherwise>
	      </xsl:choose>
	    </dd>
	    <br/>
	  </xsl:if>



	  <!-- kursa -->
	  <xsl:if test="not(normalize-space($kursa_page)='')">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="$kursa_page"/>
		</xsl:attribute>
		<xsl:value-of select="kursa"/>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:value-of select="kursa_text"/>
	    </dd>
	    <br/>
	  </xsl:if>

	  <!-- TTS -->
	  <xsl:if test="not(normalize-space($tts_page)='')">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="$tts_page"/>
		</xsl:attribute>
		<xsl:value-of select="tts"/>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:value-of select="tts_text"/>
	    </dd>
	    <br/>
	  </xsl:if>


	  <!-- gielese -->
	  <xsl:if test="not(normalize-space($gielese_page)='')">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="$gielese_page"/>
		</xsl:attribute>
		<xsl:value-of select="gielese"/>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:value-of select="gielese_text"/>
	    </dd>
	    <br/>
	  </xsl:if>

	  <!-- visl -->
	  <xsl:if test="not(normalize-space($visl_page)='')">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="$visl_page"/>
		</xsl:attribute>
		<xsl:value-of select="visl"/>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:value-of select="visl_text"/>
	    </dd>
	    <br/>
	  </xsl:if>

	  <!-- mt = jorgal.uit.no -->
	  <xsl:if test="not(normalize-space($mt_page)='')">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="$mt_page"/>
		</xsl:attribute>
		<xsl:value-of select="mt"/>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:value-of select="mt_text"/>
	    </dd>
	    <br/>
	  </xsl:if>

	  <!-- mt2 = gtweb.uit.no/mt -->
	  <xsl:if test="not(normalize-space($mt2_page)='')">
	    <dt>
	      <xsl:element name="a">
		<xsl:attribute name="href">
		  <xsl:value-of select="$mt2_page"/>
		</xsl:attribute>
		<xsl:value-of select="mt2"/>
	      </xsl:element>
	    </dt>
	    <dd>
	      <xsl:value-of select="mt2_text"/>
	    </dd>
	    <br/>
	  </xsl:if>

	</dl>

	<p>
	  <xsl:choose>
	    <xsl:when test="other[@lang = $lang ]">
	      <xsl:value-of select="other[@lang = $lang ]"/>
	    </xsl:when>
	    <xsl:otherwise>
	      <!-- <xsl:value-of select="other"/> -->
	    </xsl:otherwise>
	  </xsl:choose>
	</p>
      </body>
    </document>
  </xsl:template>

</xsl:stylesheet>
