<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_WH.onLoad(function ()     var linksButt_4de825</name>
   <tag></tag>
   <elementGuidId>f0e9a214-dc72-4e83-b59d-a9cc0fa51efa</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.text</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='main-contents']/div[2]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>text</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>


    WH.onLoad(function () {
    var linksButton = WH.ge('open-links-button');
    if (linksButton) {
        linksButton.dataset.displayId = 11470;
        linksButton.dataset.modelEntityType = 3;
        linksButton.dataset.modelEntityId = 2168;
    }
});Voir en 3DLiens

WH.Gatherer.addData(29, 1, {&quot;132537&quot;:{&quot;name&quot;:&quot;inv_boots_03&quot;,&quot;icon&quot;:&quot;inv_boots_03&quot;}});



    
    
    
        
                        En bref        
    
    
        
        
                Faction : AucunCout de réparation: 1 50Ajouté dans la mise à jour 1.11.1.5462Icône :  inv_boots_03
        
            WH.markup.printHtml(&quot;[ul][li]Faction : Aucun[\/li][li]Cout de r\u00e9paration: [money=150][\/li][li]Ajout\u00e9 dans la mise \u00e0 jour 1.11.1.5462[\/li][li class=icon-db-link]Ic\u00f4ne : [icondb=132537 name=true][\/li][\/ul]&quot;, &quot;infobox-contents-0&quot;, {
                allow: WH.markup.CLASS.STAFF,
                dbPage: true,            });
        
            
    
    
    
    
    
        
                        Captures d'écran        
    
    
        
        
                            
    
    
    
    
    
        
                        Vidéos        
    
    
        
        
                Aucune – Suggérez en un la première !            
    
    


WH.prepInfobox();

        WH.onLoad(function () {
            ss_appendSticky();
            vi_appendSticky();
        });
    
Chahuteurs de cadavre
&lt;table>&lt;tr>&lt;td>&lt;!--nstart-->&lt;!--nend-->&lt;!--ndstart-->&lt;!--ndend-->&lt;span class=&quot;q&quot;>&lt;br>Niveau d'objet &lt;!--ilvl-->16&lt;/span>&lt;!--bo-->&lt;br>Lié quand ramassé&lt;!--ue-->&lt;table width=&quot;100%&quot;>&lt;tr>&lt;td>Pieds&lt;/td>&lt;th>&lt;!--scstart4:1-->&lt;span class=&quot;q1&quot;>Tissu&lt;/span>&lt;!--scend-->&lt;/th>&lt;/tr>&lt;/table>&lt;span>&lt;!--amr-->Armure : 2&lt;/span>&lt;br>&lt;span>&lt;!--stat5-->+2 Intelligence&lt;/span>&lt;br>&lt;span>&lt;!--stat7-->+3 Endurance&lt;/span>&lt;!--ebstats-->&lt;br>&lt;span class=&quot;q2&quot;>Augmente votre score de hâte de +&lt;!--rtg36-->2.&lt;/span>&lt;br>&lt;span class=&quot;q2&quot;>+&lt;!--rtg40-->2 Versatilité&lt;/span>&lt;!--egstats-->&lt;!--eistats-->&lt;!--e-->&lt;!--ps-->&lt;br>Durabilité 60 / 60&lt;/td>&lt;/tr>&lt;/table>&lt;table>&lt;tr>&lt;td>&lt;!--itemEffects:0-->Niveau &lt;!--rlvl-->10 requis&lt;div class=&quot;whtt-sellprice&quot;>Prix de Vente: &lt;span class=&quot;moneysilver&quot;>15&lt;/span> &lt;span class=&quot;moneycopper&quot;>26&lt;/span>&lt;/div>&lt;/td>&lt;/tr>&lt;/table>

Chahuteurs de cadavreNiveau d'objet 57Lié quand ramasséPiedsTissuArmure : 2+12 Intelligence+18 EnduranceAugmente votre score de hâte de +13 (0.39% au niveau 70)+11 Versatilité (0.28% au niveau 70)Durabilité 60 / 60Niveau 10 requisPrix de Vente: 11 81 49

Niveau7070



Versions de l'objet: Augmente avec le niveau

Affichage de l'infobulle pour Choisissez une spécialisation


WH.ge('ic2168').appendChild(Icon.create('inv_boots_03', 2, null, 0, 1));
var tt = WH.ge('tt2168');
var sl = WH.ge('sl2168');
var ks = WH.ge('ks2168');
var iu = WH.ge('iu2168');
WH.updateTooltip.call(tt,
    (WH.enhanceTooltip.bind(tt))(
        2168,
        true,
        true,
        sl,
        null,
        null,
        ks,
        null,
        -1,
        iu,
        true,
        null,
        null,
        '6710'
    )
);



    
    Les joueurs peuvent gagner cet objet en choisissant les spécialisations de classe suivantes :
    
        $('#itemspec_ttip').bind('mousemove', function(e) {
            WH.Tooltip.showAtCursor(e, &quot;La sp\u00e9cialisation du butin s'applique \u00e0 la recherche de raid, aux jets bonus, aux r\u00e9compenses de qu\u00eates et des sc\u00e9narios h\u00e9ro\u00efques.&quot;, 0, 0, 'q');
        });
        $('#itemspec_ttip').bind('mouseout', function(e) {
            WH.Tooltip.hide();
        });
    
            Mage:
                        
                
                
                
            
            
                $('#lootspecs').find('.spec62 a').bind('mouseover', function() {
                    WH.Tooltip.show(this, &quot;Arcanes&quot;, 0, 0, 'q');
                });
                $('#lootspecs').find('.spec62 a').bind('mouseout', function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $('#lootspecs').find('.spec63 a').bind('mouseover', function() {
                    WH.Tooltip.show(this, &quot;Feu&quot;, 0, 0, 'q');
                });
                $('#lootspecs').find('.spec63 a').bind('mouseout', function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $('#lootspecs').find('.spec64 a').bind('mouseover', function() {
                    WH.Tooltip.show(this, &quot;Givre&quot;, 0, 0, 'q');
                });
                $('#lootspecs').find('.spec64 a').bind('mouseout', function() {
                    WH.Tooltip.hide();
                });
            
                    
            Prêtre:
                        
                
                
                
            
            
                $('#lootspecs').find('.spec256 a').bind('mouseover', function() {
                    WH.Tooltip.show(this, &quot;Discipline&quot;, 0, 0, 'q');
                });
                $('#lootspecs').find('.spec256 a').bind('mouseout', function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $('#lootspecs').find('.spec257 a').bind('mouseover', function() {
                    WH.Tooltip.show(this, &quot;Sacr\u00e9&quot;, 0, 0, 'q');
                });
                $('#lootspecs').find('.spec257 a').bind('mouseout', function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $('#lootspecs').find('.spec258 a').bind('mouseover', function() {
                    WH.Tooltip.show(this, &quot;Ombre&quot;, 0, 0, 'q');
                });
                $('#lootspecs').find('.spec258 a').bind('mouseout', function() {
                    WH.Tooltip.hide();
                });
            
                    
            Démoniste:
                        
                
                
                
            
            
                $('#lootspecs').find('.spec265 a').bind('mouseover', function() {
                    WH.Tooltip.show(this, &quot;Affliction&quot;, 0, 0, 'q');
                });
                $('#lootspecs').find('.spec265 a').bind('mouseout', function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $('#lootspecs').find('.spec266 a').bind('mouseover', function() {
                    WH.Tooltip.show(this, &quot;D\u00e9monologie&quot;, 0, 0, 'q');
                });
                $('#lootspecs').find('.spec266 a').bind('mouseout', function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $('#lootspecs').find('.spec267 a').bind('mouseover', function() {
                    WH.Tooltip.show(this, &quot;Destruction&quot;, 0, 0, 'q');
                });
                $('#lootspecs').find('.spec267 a').bind('mouseout', function() {
                    WH.Tooltip.hide();
                });
            
                    
    





Guides
    
        
        
            Level 20 F2P &quot;Twinking&quot; Tables - Outdated        
            



Informations connexes</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;main-contents&quot;)/div[@class=&quot;text&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='main-contents']/div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Pieds'])[1]/following::div[3]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Tissu'])[1]/following::div[3]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div[3]/div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;


    WH.onLoad(function () {
    var linksButton = WH.ge(&quot; , &quot;'&quot; , &quot;open-links-button&quot; , &quot;'&quot; , &quot;);
    if (linksButton) {
        linksButton.dataset.displayId = 11470;
        linksButton.dataset.modelEntityType = 3;
        linksButton.dataset.modelEntityId = 2168;
    }
});Voir en 3DLiens

WH.Gatherer.addData(29, 1, {&quot;132537&quot;:{&quot;name&quot;:&quot;inv_boots_03&quot;,&quot;icon&quot;:&quot;inv_boots_03&quot;}});



    
    
    
        
                        En bref        
    
    
        
        
                Faction : AucunCout de réparation: 1 50Ajouté dans la mise à jour 1.11.1.5462Icône :  inv_boots_03
        
            WH.markup.printHtml(&quot;[ul][li]Faction : Aucun[\/li][li]Cout de r\u00e9paration: [money=150][\/li][li]Ajout\u00e9 dans la mise \u00e0 jour 1.11.1.5462[\/li][li class=icon-db-link]Ic\u00f4ne : [icondb=132537 name=true][\/li][\/ul]&quot;, &quot;infobox-contents-0&quot;, {
                allow: WH.markup.CLASS.STAFF,
                dbPage: true,            });
        
            
    
    
    
    
    
        
                        Captures d&quot; , &quot;'&quot; , &quot;écran        
    
    
        
        
                            
    
    
    
    
    
        
                        Vidéos        
    
    
        
        
                Aucune – Suggérez en un la première !            
    
    


WH.prepInfobox();

        WH.onLoad(function () {
            ss_appendSticky();
            vi_appendSticky();
        });
    
Chahuteurs de cadavre
&lt;table>&lt;tr>&lt;td>&lt;!--nstart-->&lt;!--nend-->&lt;!--ndstart-->&lt;!--ndend-->&lt;span class=&quot;q&quot;>&lt;br>Niveau d&quot; , &quot;'&quot; , &quot;objet &lt;!--ilvl-->16&lt;/span>&lt;!--bo-->&lt;br>Lié quand ramassé&lt;!--ue-->&lt;table width=&quot;100%&quot;>&lt;tr>&lt;td>Pieds&lt;/td>&lt;th>&lt;!--scstart4:1-->&lt;span class=&quot;q1&quot;>Tissu&lt;/span>&lt;!--scend-->&lt;/th>&lt;/tr>&lt;/table>&lt;span>&lt;!--amr-->Armure : 2&lt;/span>&lt;br>&lt;span>&lt;!--stat5-->+2 Intelligence&lt;/span>&lt;br>&lt;span>&lt;!--stat7-->+3 Endurance&lt;/span>&lt;!--ebstats-->&lt;br>&lt;span class=&quot;q2&quot;>Augmente votre score de hâte de +&lt;!--rtg36-->2.&lt;/span>&lt;br>&lt;span class=&quot;q2&quot;>+&lt;!--rtg40-->2 Versatilité&lt;/span>&lt;!--egstats-->&lt;!--eistats-->&lt;!--e-->&lt;!--ps-->&lt;br>Durabilité 60 / 60&lt;/td>&lt;/tr>&lt;/table>&lt;table>&lt;tr>&lt;td>&lt;!--itemEffects:0-->Niveau &lt;!--rlvl-->10 requis&lt;div class=&quot;whtt-sellprice&quot;>Prix de Vente: &lt;span class=&quot;moneysilver&quot;>15&lt;/span> &lt;span class=&quot;moneycopper&quot;>26&lt;/span>&lt;/div>&lt;/td>&lt;/tr>&lt;/table>

Chahuteurs de cadavreNiveau d&quot; , &quot;'&quot; , &quot;objet 57Lié quand ramasséPiedsTissuArmure : 2+12 Intelligence+18 EnduranceAugmente votre score de hâte de +13 (0.39% au niveau 70)+11 Versatilité (0.28% au niveau 70)Durabilité 60 / 60Niveau 10 requisPrix de Vente: 11 81 49

Niveau7070



Versions de l&quot; , &quot;'&quot; , &quot;objet: Augmente avec le niveau

Affichage de l&quot; , &quot;'&quot; , &quot;infobulle pour Choisissez une spécialisation


WH.ge(&quot; , &quot;'&quot; , &quot;ic2168&quot; , &quot;'&quot; , &quot;).appendChild(Icon.create(&quot; , &quot;'&quot; , &quot;inv_boots_03&quot; , &quot;'&quot; , &quot;, 2, null, 0, 1));
var tt = WH.ge(&quot; , &quot;'&quot; , &quot;tt2168&quot; , &quot;'&quot; , &quot;);
var sl = WH.ge(&quot; , &quot;'&quot; , &quot;sl2168&quot; , &quot;'&quot; , &quot;);
var ks = WH.ge(&quot; , &quot;'&quot; , &quot;ks2168&quot; , &quot;'&quot; , &quot;);
var iu = WH.ge(&quot; , &quot;'&quot; , &quot;iu2168&quot; , &quot;'&quot; , &quot;);
WH.updateTooltip.call(tt,
    (WH.enhanceTooltip.bind(tt))(
        2168,
        true,
        true,
        sl,
        null,
        null,
        ks,
        null,
        -1,
        iu,
        true,
        null,
        null,
        &quot; , &quot;'&quot; , &quot;6710&quot; , &quot;'&quot; , &quot;
    )
);



    
    Les joueurs peuvent gagner cet objet en choisissant les spécialisations de classe suivantes :
    
        $(&quot; , &quot;'&quot; , &quot;#itemspec_ttip&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mousemove&quot; , &quot;'&quot; , &quot;, function(e) {
            WH.Tooltip.showAtCursor(e, &quot;La sp\u00e9cialisation du butin s&quot; , &quot;'&quot; , &quot;applique \u00e0 la recherche de raid, aux jets bonus, aux r\u00e9compenses de qu\u00eates et des sc\u00e9narios h\u00e9ro\u00efques.&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
        });
        $(&quot; , &quot;'&quot; , &quot;#itemspec_ttip&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function(e) {
            WH.Tooltip.hide();
        });
    
            Mage:
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec62 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Arcanes&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec62 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec63 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Feu&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec63 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec64 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Givre&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec64 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                    
            Prêtre:
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec256 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Discipline&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec256 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec257 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Sacr\u00e9&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec257 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec258 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Ombre&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec258 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                    
            Démoniste:
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec265 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Affliction&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec265 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec266 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;D\u00e9monologie&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec266 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec267 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Destruction&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec267 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                    
    





Guides
    
        
        
            Level 20 F2P &quot;Twinking&quot; Tables - Outdated        
            



Informations connexes&quot;) or . = concat(&quot;


    WH.onLoad(function () {
    var linksButton = WH.ge(&quot; , &quot;'&quot; , &quot;open-links-button&quot; , &quot;'&quot; , &quot;);
    if (linksButton) {
        linksButton.dataset.displayId = 11470;
        linksButton.dataset.modelEntityType = 3;
        linksButton.dataset.modelEntityId = 2168;
    }
});Voir en 3DLiens

WH.Gatherer.addData(29, 1, {&quot;132537&quot;:{&quot;name&quot;:&quot;inv_boots_03&quot;,&quot;icon&quot;:&quot;inv_boots_03&quot;}});



    
    
    
        
                        En bref        
    
    
        
        
                Faction : AucunCout de réparation: 1 50Ajouté dans la mise à jour 1.11.1.5462Icône :  inv_boots_03
        
            WH.markup.printHtml(&quot;[ul][li]Faction : Aucun[\/li][li]Cout de r\u00e9paration: [money=150][\/li][li]Ajout\u00e9 dans la mise \u00e0 jour 1.11.1.5462[\/li][li class=icon-db-link]Ic\u00f4ne : [icondb=132537 name=true][\/li][\/ul]&quot;, &quot;infobox-contents-0&quot;, {
                allow: WH.markup.CLASS.STAFF,
                dbPage: true,            });
        
            
    
    
    
    
    
        
                        Captures d&quot; , &quot;'&quot; , &quot;écran        
    
    
        
        
                            
    
    
    
    
    
        
                        Vidéos        
    
    
        
        
                Aucune – Suggérez en un la première !            
    
    


WH.prepInfobox();

        WH.onLoad(function () {
            ss_appendSticky();
            vi_appendSticky();
        });
    
Chahuteurs de cadavre
&lt;table>&lt;tr>&lt;td>&lt;!--nstart-->&lt;!--nend-->&lt;!--ndstart-->&lt;!--ndend-->&lt;span class=&quot;q&quot;>&lt;br>Niveau d&quot; , &quot;'&quot; , &quot;objet &lt;!--ilvl-->16&lt;/span>&lt;!--bo-->&lt;br>Lié quand ramassé&lt;!--ue-->&lt;table width=&quot;100%&quot;>&lt;tr>&lt;td>Pieds&lt;/td>&lt;th>&lt;!--scstart4:1-->&lt;span class=&quot;q1&quot;>Tissu&lt;/span>&lt;!--scend-->&lt;/th>&lt;/tr>&lt;/table>&lt;span>&lt;!--amr-->Armure : 2&lt;/span>&lt;br>&lt;span>&lt;!--stat5-->+2 Intelligence&lt;/span>&lt;br>&lt;span>&lt;!--stat7-->+3 Endurance&lt;/span>&lt;!--ebstats-->&lt;br>&lt;span class=&quot;q2&quot;>Augmente votre score de hâte de +&lt;!--rtg36-->2.&lt;/span>&lt;br>&lt;span class=&quot;q2&quot;>+&lt;!--rtg40-->2 Versatilité&lt;/span>&lt;!--egstats-->&lt;!--eistats-->&lt;!--e-->&lt;!--ps-->&lt;br>Durabilité 60 / 60&lt;/td>&lt;/tr>&lt;/table>&lt;table>&lt;tr>&lt;td>&lt;!--itemEffects:0-->Niveau &lt;!--rlvl-->10 requis&lt;div class=&quot;whtt-sellprice&quot;>Prix de Vente: &lt;span class=&quot;moneysilver&quot;>15&lt;/span> &lt;span class=&quot;moneycopper&quot;>26&lt;/span>&lt;/div>&lt;/td>&lt;/tr>&lt;/table>

Chahuteurs de cadavreNiveau d&quot; , &quot;'&quot; , &quot;objet 57Lié quand ramasséPiedsTissuArmure : 2+12 Intelligence+18 EnduranceAugmente votre score de hâte de +13 (0.39% au niveau 70)+11 Versatilité (0.28% au niveau 70)Durabilité 60 / 60Niveau 10 requisPrix de Vente: 11 81 49

Niveau7070



Versions de l&quot; , &quot;'&quot; , &quot;objet: Augmente avec le niveau

Affichage de l&quot; , &quot;'&quot; , &quot;infobulle pour Choisissez une spécialisation


WH.ge(&quot; , &quot;'&quot; , &quot;ic2168&quot; , &quot;'&quot; , &quot;).appendChild(Icon.create(&quot; , &quot;'&quot; , &quot;inv_boots_03&quot; , &quot;'&quot; , &quot;, 2, null, 0, 1));
var tt = WH.ge(&quot; , &quot;'&quot; , &quot;tt2168&quot; , &quot;'&quot; , &quot;);
var sl = WH.ge(&quot; , &quot;'&quot; , &quot;sl2168&quot; , &quot;'&quot; , &quot;);
var ks = WH.ge(&quot; , &quot;'&quot; , &quot;ks2168&quot; , &quot;'&quot; , &quot;);
var iu = WH.ge(&quot; , &quot;'&quot; , &quot;iu2168&quot; , &quot;'&quot; , &quot;);
WH.updateTooltip.call(tt,
    (WH.enhanceTooltip.bind(tt))(
        2168,
        true,
        true,
        sl,
        null,
        null,
        ks,
        null,
        -1,
        iu,
        true,
        null,
        null,
        &quot; , &quot;'&quot; , &quot;6710&quot; , &quot;'&quot; , &quot;
    )
);



    
    Les joueurs peuvent gagner cet objet en choisissant les spécialisations de classe suivantes :
    
        $(&quot; , &quot;'&quot; , &quot;#itemspec_ttip&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mousemove&quot; , &quot;'&quot; , &quot;, function(e) {
            WH.Tooltip.showAtCursor(e, &quot;La sp\u00e9cialisation du butin s&quot; , &quot;'&quot; , &quot;applique \u00e0 la recherche de raid, aux jets bonus, aux r\u00e9compenses de qu\u00eates et des sc\u00e9narios h\u00e9ro\u00efques.&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
        });
        $(&quot; , &quot;'&quot; , &quot;#itemspec_ttip&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function(e) {
            WH.Tooltip.hide();
        });
    
            Mage:
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec62 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Arcanes&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec62 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec63 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Feu&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec63 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec64 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Givre&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec64 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                    
            Prêtre:
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec256 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Discipline&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec256 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec257 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Sacr\u00e9&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec257 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec258 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Ombre&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec258 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                    
            Démoniste:
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec265 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Affliction&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec265 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec266 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;D\u00e9monologie&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec266 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                        
                
                
                
            
            
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec267 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseover&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.show(this, &quot;Destruction&quot;, 0, 0, &quot; , &quot;'&quot; , &quot;q&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;#lootspecs&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.spec267 a&quot; , &quot;'&quot; , &quot;).bind(&quot; , &quot;'&quot; , &quot;mouseout&quot; , &quot;'&quot; , &quot;, function() {
                    WH.Tooltip.hide();
                });
            
                    
    





Guides
    
        
        
            Level 20 F2P &quot;Twinking&quot; Tables - Outdated        
            



Informations connexes&quot;))]</value>
   </webElementXpaths>
</WebElementEntity>
