<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>4_Add_Offer Catalogue</name>
   <tag></tag>
   <elementGuidId>068ada76-e38a-4c3e-8361-25e3038145e5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;OfferQueryBuildFromCatalogOfferings\&quot;: {\n        \&quot;BuildFromCatalogOfferingsRequest\&quot;: {\n            \&quot;@type\&quot;: \&quot;BuildFromCatalogOfferingsRequestAir\&quot;,\n            \&quot;CatalogOfferingsIdentifier\&quot;: {\n                \&quot;Identifier\&quot;: {\n                    \&quot;value\&quot;: \&quot;${catalogOfferingsIdentifier}\&quot;\n                }\n            },\n            \&quot;CatalogOfferingIdentifier\&quot;: {\n                \&quot;Identifier\&quot;: {\n                    \&quot;value\&quot;: \&quot;o0.0\&quot;\n                }\n            },\n            \&quot;ProductIdentifier\&quot;: [\n                {\n                    \&quot;Identifier\&quot;: {\n                        \&quot;value\&quot;: \&quot;p0\&quot;\n                    }\n                }\n            ]\n        },\n        \&quot;FareRuleCategory\&quot;: [],\n        \&quot;PaymentCriteria\&quot;: {}\n    }\n}&quot;,
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
      <webElementGuid>8b00ce47-d149-4c6e-9c0f-33e45cd4a856</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>12f73430-a80b-4cb4-8d4e-940722cb8ca8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>dd3e53f4-5dac-405b-a6d9-aef325d126c0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic VW5pdmVyc2FsIEFQSS91QVBJNDM1NDgyNTAwMC0yNDk1YTRlNjpwN0EzeVV6RA==</value>
      <webElementGuid>7d8ab92b-e612-4e54-b6d0-a51c02f21f05</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>Samurai_Test</value>
      <webElementGuid>9f111b00-3289-4f18-9012-e8d3206910d7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>8be5544c-691f-4a1f-965d-f2f78840e46e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>0f21730d-b501-4aa2-ac46-9ae93bff1677</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>61bd99c7-69c4-4148-9867-2654d87846df</webElementGuid>
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
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
