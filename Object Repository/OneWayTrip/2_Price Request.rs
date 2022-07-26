<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>2_Price Request</name>
   <tag></tag>
   <elementGuidId>3d4cf4f3-527e-4534-a8e0-c423dddd0fe8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;OfferQueryBuildFromCatalogOfferings\&quot;: {\n        \&quot;BuildFromCatalogOfferingsRequest\&quot;: {\n            \&quot;@type\&quot;: \&quot;BuildFromCatalogOfferingsRequestAir\&quot;,\n            \&quot;CatalogOfferingsIdentifier\&quot;: {\n                \&quot;Identifier\&quot;: {\n                    \&quot;value\&quot;: \&quot;${sSearchToken}\&quot;\n                }\n            },\n            \&quot;CatalogOfferingIdentifier\&quot;: {\n                \&quot;Identifier\&quot;: {\n                    \&quot;value\&quot;: \&quot;o0.0\&quot;\n                }\n            },\n            \&quot;ProductIdentifier\&quot;: [\n                {\n                    \&quot;Identifier\&quot;: {\n                        \&quot;value\&quot;: \&quot;p0\&quot;\n                    }\n                }\n            ]\n        },\n        \&quot;CabinPreference\&quot;: {\n            \&quot;preferenceType\&quot;: \&quot;Preferred\&quot;,\n            \&quot;cabins\&quot;: [\n                \&quot;Economy\&quot;\n            ]\n        },\n        \&quot;FareRuleCategory\&quot;: [],\n        \&quot;PaymentCriteria\&quot;: {}\n    }\n}&quot;,
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
      <webElementGuid>0f26b215-91fb-4a50-92d4-09dbd85fd872</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a6a52211-fc65-4c1a-8f85-98b78af17d6b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic VW5pdmVyc2FsIEFQSS91QVBJNDM1NDgyNTAwMC0yNDk1YTRlNjpwN0EzeVV6RA==</value>
      <webElementGuid>4996d4f9-87ce-4ce5-940e-613c61d26c9a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>Samurai_Test</value>
      <webElementGuid>81e33657-5b69-4e13-8c2e-249c64a7587d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>feba2f4d-bf97-44f6-9651-749465abe723</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>9a6afc0e-e16a-418b-9286-3fa3c386c475</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>c55bc588-224a-4fc9-87d9-9e182da27c47</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>4ed61fa6-5fd6-42cd-b300-ce60a08bfb1b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sPriceURL}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sSearchToken</defaultValue>
      <description></description>
      <id>fa4defec-fe2f-4d55-9dae-8cae7c630841</id>
      <masked>false</masked>
      <name>sSearchToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAccessGroup</defaultValue>
      <description></description>
      <id>d94aed29-0777-4049-8258-ca14ec66e061</id>
      <masked>false</masked>
      <name>sAccessGroup</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sPriceURL</defaultValue>
      <description></description>
      <id>a87ca056-0691-4701-9c72-b3227c7acbf1</id>
      <masked>false</masked>
      <name>sPriceURL</name>
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
