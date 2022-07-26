<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>4_Add_Offer Catalogue_9Passengers</name>
   <tag></tag>
   <elementGuidId>bd7b150f-a8f3-4187-8bc4-8156d7a6f2f0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;OfferQueryBuildFromCatalogOfferings\&quot;: {\n        \&quot;BuildFromCatalogOfferingsRequest\&quot;: {\n            \&quot;@type\&quot;: \&quot;BuildFromCatalogOfferingsRequestAir\&quot;,\n            \&quot;CatalogOfferingsIdentifier\&quot;: {\n                \&quot;Identifier\&quot;: {\n                    \&quot;value\&quot;: \&quot;${catalogOfferingsIdentifier}\&quot;\n                }\n            },\n            \&quot;CatalogOfferingIdentifier\&quot;: {\n                \&quot;Identifier\&quot;: {\n                    \&quot;value\&quot;: \&quot;${CatalogOfferingIdentifierID}\&quot;\n                }\n            },\n            \&quot;ProductIdentifier\&quot;: [\n                {\n                    \&quot;Identifier\&quot;: {\n                        \&quot;value\&quot;: \&quot;${ProductFromID}\&quot;\n                    }\n                }\n            ]\n        },\n        \&quot;FareRuleCategory\&quot;: [],\n        \&quot;PaymentCriteria\&quot;: {}\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>2e588d71-1d6b-4e06-bf9f-a2b720f2089f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>67509387-9394-4661-9a8f-75ae757fa7a6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>e58a5d84-22a7-44bf-bef6-99da056c5630</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic VW5pdmVyc2FsIEFQSS91QVBJNDM1NDgyNTAwMC0yNDk1YTRlNjpwN0EzeVV6RA==</value>
      <webElementGuid>2e8e6839-9573-4b44-8a64-fc027ce0e207</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>Samurai_Test</value>
      <webElementGuid>6ee9aaa7-8485-4588-a9bd-12d6e0e84766</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>d0b63306-2311-486f-a490-e40e73220f73</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>48867110-2790-4258-8223-f72ab53461b6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>04998fc4-159b-41a0-9436-4ac21bc37cd8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sAddofferCatalogueURL}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.reservationId</defaultValue>
      <description></description>
      <id>5aa77bf1-28bf-4cba-9533-11c10141d1bd</id>
      <masked>false</masked>
      <name>reservationId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.catalogOfferingsIdentifier</defaultValue>
      <description></description>
      <id>5b5f8c7c-cac2-4430-a340-e486dc561f4d</id>
      <masked>false</masked>
      <name>catalogOfferingsIdentifier</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAccessGroup</defaultValue>
      <description></description>
      <id>01caa6a9-2991-41f8-96e5-68a4bcd0c9a8</id>
      <masked>false</masked>
      <name>sAccessGroup</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAddofferCatalogueURL</defaultValue>
      <description></description>
      <id>c0aef54a-d06c-406b-89df-5a4e1eae7ee1</id>
      <masked>false</masked>
      <name>sAddofferCatalogueURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.CatalogOfferingIdentifierID</defaultValue>
      <description></description>
      <id>7ddae8d9-ac2a-4435-9481-899f0d1049fa</id>
      <masked>false</masked>
      <name>CatalogOfferingIdentifierID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ProductFromID</defaultValue>
      <description></description>
      <id>2ee3416d-b95b-4742-a50f-26902f520800</id>
      <masked>false</masked>
      <name>ProductFromID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ProductToID</defaultValue>
      <description></description>
      <id>b11cf1b6-f124-438e-881b-a7eacccb5bc7</id>
      <masked>false</masked>
      <name>ProductToID</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
