<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>9_Add Payment</name>
   <tag></tag>
   <elementGuidId>bb07653f-e0f2-4f05-a28c-7eb122d5235d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;Payment\&quot;: {\n        \&quot;id\&quot;: \&quot;payment_1\&quot;,\n        \&quot;Identifier\&quot;: {\n            \&quot;authority\&quot;: \&quot;Travelport\&quot;,\n            \&quot;value\&quot;: \&quot;A0656EFF-FAF4-456F-B061-0161008D6A5E\&quot;\n        },\n        \&quot;Amount\&quot;: {\n            \&quot;code\&quot;: \&quot;AUD\&quot;,\n            \&quot;minorUnit\&quot;: 2,\n            \&quot;currencySource\&quot;: \&quot;Charged\&quot;,\n            \&quot;approximateInd\&quot;: true,\n            \&quot;value\&quot;: 63.1\n        },\n        \&quot;FormOfPaymentIdentifier\&quot;: {\n            \&quot;id\&quot;: \&quot;formOfPayment_5\&quot;,\n            \&quot;FormOfPaymentRef\&quot;: \&quot;formOfPayment_5\&quot;,\n            \&quot;Identifier\&quot;: {\n                \&quot;authority\&quot;: \&quot;Travelport\&quot;,\n                \&quot;value\&quot;: \&quot;A0656EFF-FAF4-456F-B061-0161008D6FOP\&quot;\n            }\n        },\n        \&quot;OfferIdentifier\&quot;: [\n            {\n                \&quot;id\&quot;: \&quot;offer_1\&quot;,\n                \&quot;offerRef\&quot;: \&quot;offer_1\&quot;,\n                \&quot;Identifier\&quot;: {\n                    \&quot;authority\&quot;: \&quot;Travelport\&quot;,\n                    \&quot;value\&quot;: \&quot;07664bb3-37a0-4c63-a2ef-ed71b5f48d49\&quot;\n                }\n            }\n        ]\n    }\n}&quot;,
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
      <webElementGuid>ff91f0e4-11fd-421b-9d8b-022b78f5eaf3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a866b68d-0dcf-45d3-80b0-b8f6cca8b203</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>64e13c55-5f65-445d-9224-1f49d2fcaefb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic VW5pdmVyc2FsIEFQSS91QVBJNDM1NDgyNTAwMC0yNDk1YTRlNjpwN0EzeVV6RA==</value>
      <webElementGuid>317c9971-b342-4bac-96a4-8298ae9e67b6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>Samurai_Test</value>
      <webElementGuid>82584c63-be35-4903-9792-a61191ee2fa7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>c725be2e-e482-4eb6-8a5b-99e1dcdc8559</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>b8d7fb4b-ccdb-4214-9179-8abff76f8a83</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>11e841b8-c7c4-4d58-8808-3ce462608af7</webElementGuid>
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
