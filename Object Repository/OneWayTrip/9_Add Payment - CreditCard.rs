<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>9_Add Payment - CreditCard</name>
   <tag></tag>
   <elementGuidId>62ceb5d9-cdb8-4008-a585-9560e841ab1b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;Payment\&quot;: {\n        \&quot;id\&quot;: \&quot;payment_1\&quot;,\n        \&quot;Identifier\&quot;: {\n            \&quot;authority\&quot;: \&quot;Travelport\&quot;,\n            \&quot;value\&quot;: \&quot;A0656EFF-FAF4-456F-B061-0161008D6A5E\&quot;\n        },\n        \&quot;Amount\&quot;: {\n            \&quot;code\&quot;: \&quot;${sCurrencyCode}\&quot;,\n            \&quot;minorUnit\&quot;: 2,\n            \&quot;currencySource\&quot;: \&quot;Charged\&quot;,\n            \&quot;approximateInd\&quot;: true,\n            \&quot;value\&quot;: 63.1\n        },\n        \&quot;FormOfPaymentIdentifier\&quot;: {\n            \&quot;id\&quot;: \&quot;formOfPayment_4\&quot;,\n            \&quot;FormOfPaymentRef\&quot;: \&quot;formOfPayment_4\&quot;,\n            \&quot;Identifier\&quot;: {\n                \&quot;authority\&quot;: \&quot;Travelport\&quot;,\n                \&quot;value\&quot;: \&quot;A0656EFF-FAF4-456F-B061-0161008D6FOP\&quot;\n            }\n        },\n        \&quot;OfferIdentifier\&quot;: [\n            {\n                \&quot;id\&quot;: \&quot;offer_1\&quot;,\n                \&quot;offerRef\&quot;: \&quot;offer_1\&quot;,\n                \&quot;Identifier\&quot;: {\n                    \&quot;authority\&quot;: \&quot;Travelport\&quot;,\n                    \&quot;value\&quot;: \&quot;${ExchSearchIdentifierValue}\&quot;\n                }\n            }\n        ]\n    }\n}&quot;,
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
      <webElementGuid>6072d71b-5017-4ca9-a8c6-3efe42dc0b4c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>05e498f2-4c02-4605-a472-a596ceabbfd8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>80a8296b-7921-4bfa-b294-6bcfee7525cf</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic VW5pdmVyc2FsIEFQSS91QVBJNDM1NDgyNTAwMC0yNDk1YTRlNjpwN0EzeVV6RA==</value>
      <webElementGuid>456fd3fc-77f1-41dd-b186-b93ea10a02ea</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>Samurai_Test</value>
      <webElementGuid>7b4f45b6-4ed1-4b9a-a430-4f71f2ec9410</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>5c9ce578-c588-4097-b9d9-65c0d51cb51e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>a31ffb52-c4c7-45ae-bdd7-dfdf14a5716b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>ebfe4ccc-d260-42a8-bc87-930482a3010f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sAddPaymentURL}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.eCacheId</defaultValue>
      <description></description>
      <id>70aeea21-26e0-46e2-a5ad-88b3aad1e914</id>
      <masked>false</masked>
      <name>eCacheId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAccessGroup</defaultValue>
      <description></description>
      <id>f44b82b2-4b4d-4c46-8d6c-80376e39022b</id>
      <masked>false</masked>
      <name>sAccessGroup</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAddPaymentURL</defaultValue>
      <description></description>
      <id>f7d87f92-eb80-4639-b789-3d38cbbd3898</id>
      <masked>false</masked>
      <name>sAddPaymentURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sCurrencyCode</defaultValue>
      <description></description>
      <id>f55c2a0e-6938-424b-882d-936d0ebab384</id>
      <masked>false</masked>
      <name>sCurrencyCode</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ExchSearchIdentifierValue</defaultValue>
      <description></description>
      <id>13ba5162-0043-4ccf-bc48-826afe0ac5b3</id>
      <masked>false</masked>
      <name>ExchSearchIdentifierValue</name>
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
