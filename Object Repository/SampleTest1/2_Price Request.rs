<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>2_Price Request</name>
   <tag></tag>
   <elementGuidId>e2168f7b-5e16-437d-8903-61d0793e3770</elementGuidId>
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
      <webElementGuid>8758b362-c307-4bbd-a71b-1ae5a98f332a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>f5162fe7-2876-44df-a221-e1143ec00ab6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic VW5pdmVyc2FsIEFQSS91QVBJNDM1NDgyNTAwMC0yNDk1YTRlNjpwN0EzeVV6RA==</value>
      <webElementGuid>3a35483e-241d-4d9e-aa69-ec095638d4a5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>Samurai_Test</value>
      <webElementGuid>4a7dd28f-5d8f-4c8e-9afc-6b350f92a915</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>eb2e6bd4-e083-4d57-a6c1-2f40a86f0653</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>dfee9f9e-a3b7-410d-9a78-42ad3a73be80</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>59af1383-8856-4cd7-a0cc-a23b5a456462</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>7C7ED10A-EEBC-4468-B499-879DE63F1B7D</value>
      <webElementGuid>1e9be659-2e3a-4878-9a07-e210366dff59</webElementGuid>
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
