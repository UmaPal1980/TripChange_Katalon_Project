<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>X_Add Payment PNR</name>
   <tag></tag>
   <elementGuidId>aed6a09f-6965-4121-ba0e-0c30af5de426</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;Payment\&quot; : {\n    \&quot;id\&quot; : \&quot;payment_1\&quot;,\n    \&quot;Identifier\&quot; : {\n      \&quot;authority\&quot; : \&quot;Travelport\&quot;,\n      \&quot;value\&quot; : \&quot;A0656EFF-FAF4-456F-B061-0161008D6A5E\&quot;\n    },\n   \&quot;Amount\&quot;: {\n                        \&quot;code\&quot;: \&quot;USD\&quot;,\n                        \&quot;value\&quot;: \&quot;462.33\&quot;\n                    },\n    \&quot;FormOfPaymentIdentifier\&quot; : {\n      \&quot;id\&quot; : \&quot;formOfPayment_1\&quot;,\n      \&quot;FormOfPaymentRef\&quot; : \&quot;formOfPayment_1\&quot;,\n      \&quot;Identifier\&quot; : {\n        \&quot;authority\&quot; : \&quot;Travelport\&quot;,\n        \&quot;value\&quot; : \&quot;${sOfferIdentifierValue}\&quot;\n      }\n    },\n    \&quot;OfferIdentifier\&quot; : [ {\n   \&quot;id\&quot; : \&quot;offerModify_1\&quot;,\n\&quot;offerRef\&quot; : \&quot;offerModify_1\&quot;,\n      \&quot;Identifier\&quot; : {\n        \&quot;authority\&quot; : \&quot;Travelport\&quot;,\n        \&quot;value\&quot; : \&quot;${ExchSearchIdentifierValue}\&quot;\n      }\n    } ]\n  }\n}&quot;,
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
      <webElementGuid>c00bc256-c18e-4d52-9e9a-f576e6937f08</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b48379a4-2e82-4936-9525-19fae23de2f5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>ec16d0e3-020e-471a-b80d-b9dfd72f5b7d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic VW5pdmVyc2FsIEFQSS91QVBJNDM1NDgyNTAwMC0yNDk1YTRlNjpwN0EzeVV6RA==</value>
      <webElementGuid>1e73fa17-f995-4ce7-82fa-a0cf55a91718</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>Samurai_Test</value>
      <webElementGuid>e6a756ee-f7f5-4728-9f1e-caa26ea3bf3d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>04c1fdb5-d594-4de0-aac3-0baa7a6e0a17</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>Locale=en_US,lastName=testuser,accessGroups=cn=512928,ou=Organization,dc=travelport,dc=com^cn=504860,ou=Organization,dc=travelport,dc=com^cn=AB6922CC-BE07-4291-B38F-AB25ED96F07E,ou=AccessGroups,dc=travelport,dc=com^cn=7C7ED10A-EEBC-4468-B499-879DE63F1B7D,ou=AccessGroups,dc=travelport,dc=com^cn=D6BBCAA7-76AE-493F-87DD-7D32F12EE31B,ou=AccessGroups,dc=travelport,dc=com,accessList=512928^504860^AB6922CC-BE07-4291-B38F-AB25ED96F07E^7C7ED10A-EEBC-4468-B499-879DE63F1B7D^D6BBCAA7-76AE-493F-87DD-7D32F12EE31B,Roles=Developer^Travel Agent,SSOSessionProperties={\\\&quot;SmAuthResults\\\&quot;:\\\&quot;AQFzABVUUzFHdGVzdHVzZXJAbWFpbC5jb21zABxOdEk3K1FKOWs1RVowbkJaYzBaNXZNUnNMYms9cwHsYXUyMUhxeEdaTW1LTXlOYUYwN3FtRTlscG5IRVdnM0hqTlYvQ0IzMzdqeHhjekJMK3FCSGpjdTNWVllaTEtzNW1wLzVSUGYxVmo3S1pVbGI2ODlDYS9kTTE5YW8rRWlHaHhUQTNKcnVlZloyR1p0NjRZQXU5Y0RNSlkzVTB4L1pLeG5mZmpQY3Zubm41Y0hvb2hDaU5rRVBvRmZQTWMwSDVjdDN5ZnlVUVZoZVVGUU43NGRaeEdxSjhtN1ZhdDlzNEZPeVZ1Wk10dVBQZ1JPWncrQ3JNc2c3TTVGNDlsWDljTyszdWRJTWhBWExkekV4MVpGeWpudHBXSjY0RERZR2JvQU0xbGlIbHI1eUlVdE1XUjJoMDJZK0Y5UXhra1ZqbStRck0wL295bUlSZzFiSG9BZGNRVm1XNE5rVkZMc1pzMGw3MDJ1V0JIRHVTOURkUG5CU0c3bSszWVlpV2gycTdIdjMzVmFMbWUvNVVPLzJiY3g2T0M1TFE3eVdWNUZOcWJEa1Y3RlFua0x6MmxiVjdIR20zMnltRzlwSFNESkRlTnYwZ2lMcm4xR0tmcmY4YXg4cko2Q0p4S1hhN3F1aFJ0cldva1h2aWN2NkpmWWduVUN3dE1VRGlXT2ZiMEs1b0JNaEdUc0o0Q3c9CAAAAXVXTpCAAjhAAnCABF+S1tAEX5LW0ARfktbS\\\&quot;},firstName=TRIPSERVICES,UID=TP98348858,Language=English,Static_Gtids=,email=TS1Gtestuser@mail.com, forwarded=for=10.107.131.31;host=ts-airbook-res-session-do-6-qab-tripservices-qa.ocp-a.zu2.nonprod.travelport.io:443;proto=https;proto-version=, xauth_travelport_accessgroup=7C7ED10A-EEBC-4468-B499-879DE63F1B7D, x-forwarded-port=443, e2etrackingid=IntegratedDemo:qand1001}</value>
      <webElementGuid>a78580b3-0ab6-41e2-8ea3-d0512c46f78b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAuth_ClientID</name>
      <type>Main</type>
      <value>qa-sXZ3EygiKuovA7RgGjDbZ1PIRimxetbaAGczeGnW</value>
      <webElementGuid>825a0ed4-4fd9-4a2b-9c8d-1516d9928e75</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>84c88499-10f9-44c6-a015-cb406af6f17f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sAddPaymentExchURL}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sAccessGroup</defaultValue>
      <description></description>
      <id>b7503055-668c-4a86-ad10-6ff082d51343</id>
      <masked>false</masked>
      <name>sAccessGroup</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.IdentifierResWorkBench</defaultValue>
      <description></description>
      <id>1144770a-2ba0-4eb1-b5ed-a3991b01d7d1</id>
      <masked>false</masked>
      <name>IdentifierResWorkBench</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sOfferIdentifierValue</defaultValue>
      <description></description>
      <id>11664498-221f-4563-b234-91e3b4546440</id>
      <masked>false</masked>
      <name>sOfferIdentifierValue</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ExchSearchIdentifierValue</defaultValue>
      <description></description>
      <id>3ff35200-3f07-43ce-91a7-ba6dd0600291</id>
      <masked>false</masked>
      <name>ExchSearchIdentifierValue</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAddPaymentExchURL</defaultValue>
      <description></description>
      <id>e03ae376-8c5a-4801-be3d-77729b1544a3</id>
      <masked>false</masked>
      <name>sAddPaymentExchURL</name>
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
